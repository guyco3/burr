use crate::policy;
use anyhow::{Context, Result};
use futures_util::TryStreamExt;
use js_component_bindgen::{transpile, AsyncMode, TranspileOpts};
use oci_client::{Client, Reference};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use wasm_pkg_client::Client as WkgClient;
use wasm_pkg_common::package::{PackageRef, Version};

const VIRTUALIZER_WASM: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/virtualizer.wasm"));

const WARDEN_SHIM_JS: &str = include_str!("assets/warden_shim.js");

#[derive(Debug, PartialEq)]
pub enum ParsedReference {
    Local(PathBuf, String),
    Oci(Reference, String),
    Wkg(PackageRef, String),
}

pub fn parse_reference(oci_ref: &str) -> Result<ParsedReference> {
    if let Some(local_path) = oci_ref.strip_prefix("file://") {
        let path = Path::new(local_path);
        let pkg_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        Ok(ParsedReference::Local(PathBuf::from(local_path), pkg_name))
    } else if let Ok(package_ref) = PackageRef::from_str(oci_ref) {
        let pkg_name = package_ref
            .clone()
            .to_string()
            .replace(":", "_")
            .replace("/", "_");
        Ok(ParsedReference::Wkg(package_ref, pkg_name))
    } else {
        let reference: Reference = oci_ref.parse().context("Invalid OCI reference")?;
        let pkg_name = reference.repository().replace("/", "_");
        Ok(ParsedReference::Oci(reference, pkg_name))
    }
}

pub async fn run_install(oci_ref: &str) -> Result<()> {
    let parsed = parse_reference(oci_ref)?;
    let pkg_name = match &parsed {
        ParsedReference::Local(_, pkg) => pkg.clone(),
        ParsedReference::Oci(_, pkg) => pkg.clone(),
        ParsedReference::Wkg(_, pkg) => pkg.clone(),
    };

    let wrdn_dir = PathBuf::from(".wrdn");
    let pkg_dir = wrdn_dir.join(&pkg_name);
    fs::create_dir_all(&pkg_dir).context("Failed to create .wrdn pkg directory")?;
    let guest_wasm_path = pkg_dir.join("guest.wasm");

    match parsed {
        ParsedReference::Local(local_path, _) => {
            fs::copy(&local_path, &guest_wasm_path).context("Failed to copy local guest wasm")?;
            log::info!("Using local guest wasm from {}...", local_path.display());
        }
        ParsedReference::Oci(reference, _) => {
            log::info!("Pulling guest wasm from {}...", oci_ref);
            let client = Client::new(oci_client::client::ClientConfig::default());
            let image_data = client
                .pull(
                    &reference,
                    &oci_client::secrets::RegistryAuth::Anonymous,
                    vec![
                        "application/vnd.wasm.component.v1+wasm",
                        "application/vnd.oci.image.layer.v1.tar+gzip",
                        "application/wasm",
                    ],
                )
                .await
                .context("Failed to pull OCI artifact")?;

            if let Some(layer) = image_data.layers.first() {
                fs::write(&guest_wasm_path, &layer.data).context("Failed to write guest wasm")?;
            } else {
                anyhow::bail!("No layers found in the OCI artifact");
            }
        }
        ParsedReference::Wkg(package_ref, _) => {
            log::info!("Resolving Wasm package {}...", oci_ref);
            let client = WkgClient::with_global_defaults()
                .await
                .context("Failed to load global wkg config")?;

            let version = match None::<Version> /* TODO handle version */ {
                Some(ref ver) => ver.clone(),
                None => {
                    log::info!("No version specified, fetching latest...");
                    let versions = client.list_all_versions(&package_ref.clone()).await.context("Failed to list versions")?;
                    versions.into_iter()
                        .filter_map(|vi| (!vi.yanked).then_some(vi.version))
                        .max()
                        .context("No releases found")?
                }
            };

            log::info!(
                "Pulling Wasm component {}@{}...",
                package_ref.clone(),
                version
            );
            let release = client
                .get_release(&package_ref.clone(), &version)
                .await
                .context("Failed to get release details")?;

            let mut stream = client
                .stream_content(&package_ref.clone(), &release)
                .await
                .context("Failed to stream component")?;
            let mut file = tokio::fs::File::create(&guest_wasm_path)
                .await
                .context("Failed to create guest wasm file")?;
            while let Some(chunk) = stream
                .try_next()
                .await
                .context("Error reading component stream")?
            {
                file.write_all(&chunk)
                    .await
                    .context("Error writing component chunk")?;
            }
        }
    }

    log::info!("Writing embedded virtualizer...");
    let virtualizer_path = pkg_dir.join("virtualizer.wasm");
    fs::write(&virtualizer_path, VIRTUALIZER_WASM).context("Failed to write virtualizer wasm")?;

    log::info!("Transpiling virtualizer...");
    let mut virt_opts = TranspileOpts::default();
    virt_opts.name = "virtualizer".to_string();
    virt_opts.async_mode = Some(AsyncMode::JavaScriptPromiseIntegration {
        imports: vec![],
        exports: vec![],
    });

    let virt_wasm = fs::read(&virtualizer_path).context("Failed to read virtualizer wasm")?;
    let virt_transpiled =
        transpile(&virt_wasm, virt_opts).context("Failed to transpile virtualizer")?;

    let virt_out_dir = pkg_dir.join("out-warden");
    fs::create_dir_all(&virt_out_dir)?;
    for (file_name, data) in virt_transpiled.files {
        fs::write(virt_out_dir.join(file_name), data)?;
    }

    log::info!("Creating module mapper shim...");
    let shim_path = pkg_dir.join("warden_shim.js");
    fs::write(&shim_path, WARDEN_SHIM_JS).context("Failed to write warden shim")?;

    log::info!("Transpiling guest with mappings...");
    let shim_rel_path = "../warden_shim.js";
    let mut guest_map = HashMap::new();
    guest_map.insert(
        "wasi:cli/environment@0.3.0".to_string(),
        shim_rel_path.to_string(),
    );
    guest_map.insert(
        "wasi:filesystem/preopens@0.3.0".to_string(),
        shim_rel_path.to_string(),
    );
    guest_map.insert(
        "wasi:filesystem/types@0.3.0".to_string(),
        shim_rel_path.to_string(),
    );
    guest_map.insert(
        "wasi:sockets/types@0.3.0".to_string(),
        shim_rel_path.to_string(),
    );
    guest_map.insert(
        "wasi:sockets/ip-name-lookup@0.3.0".to_string(),
        shim_rel_path.to_string(),
    );

    let mut guest_opts = TranspileOpts::default();
    guest_opts.name = "guest".to_string();
    guest_opts.map = Some(guest_map);
    guest_opts.async_mode = Some(AsyncMode::JavaScriptPromiseIntegration {
        imports: vec![],
        exports: vec![],
    });

    let guest_wasm = fs::read(&guest_wasm_path).context("Failed to read guest wasm")?;
    let guest_transpiled =
        transpile(&guest_wasm, guest_opts).context("Failed to transpile guest")?;

    let guest_out_dir = pkg_dir.join("out-guest");
    fs::create_dir_all(&guest_out_dir)?;
    for (file_name, data) in guest_transpiled.files {
        fs::write(guest_out_dir.join(file_name), data)?;
    }

    policy::ensure_policy_exists(&pkg_dir)?;

    log::info!("Generating Node.js entrypoint...");
    let index_js_path = pkg_dir.join("index.js");
    let index_js_content = include_str!("assets/index.js");
    fs::write(&index_js_path, index_js_content).context("Failed to write index.js entrypoint")?;

    log::info!("Installation complete for {}.", oci_ref);
    log::info!(
        "To use this package, import from '.wrdn/{}/index.js'",
        pkg_name
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_local_reference() {
        let parsed = parse_reference("file:///tmp/guest.wasm").unwrap();
        assert_eq!(
            parsed,
            ParsedReference::Local(PathBuf::from("/tmp/guest.wasm"), "guest".to_string())
        );
    }

    #[test]
    fn test_parse_oci_reference() {
        let parsed = parse_reference("ghcr.io/guyco3/guest:latest").unwrap();
        if let ParsedReference::Oci(reference, pkg_name) = parsed {
            assert_eq!(reference.repository(), "guyco3/guest");
            assert_eq!(pkg_name, "guyco3_guest");
        } else {
            panic!("Expected ParsedReference::Oci");
        }
    }
}
