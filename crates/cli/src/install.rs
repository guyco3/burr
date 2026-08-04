use anyhow::{Context, Result};
use oci_client::{Client, Reference};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::policy;

const VIRTUALIZER_WASM: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/virtualizer.wasm"));

const WARDEN_SHIM_JS: &str = r#"
import * as warden from './out-warden/virtualizer.js';

const env = warden['wasi:cli/environment@0.3.0'];
export const getEnvironment = env?.getEnvironment;
export const getArguments = env?.getArguments;
export const initialCwd = env?.initialCwd;

const preopens = warden['wasi:filesystem/preopens@0.3.0'];
export const getDirectories = preopens?.getDirectories;

const fsTypes = warden['wasi:filesystem/types@0.3.0'];
export const Descriptor = fsTypes?.Descriptor;

const sockTypes = warden['wasi:sockets/types@0.3.0'];
export const TcpSocket = sockTypes?.TcpSocket;
export const UdpSocket = sockTypes?.UdpSocket;

const ipLookup = warden['wasi:sockets/ip-name-lookup@0.3.0'];
export const resolveAddresses = ipLookup?.resolveAddresses;
"#;

#[derive(Debug, PartialEq)]
pub enum ParsedReference {
    Local(PathBuf, String),
    Oci(Reference, String),
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
    };
    
    let wrdn_dir = PathBuf::from(".wrdn");
    let pkg_dir = wrdn_dir.join(&pkg_name);
    fs::create_dir_all(&pkg_dir).context("Failed to create .wrdn pkg directory")?;
    let guest_wasm_path = pkg_dir.join("guest.wasm");

    match parsed {
        ParsedReference::Local(local_path, _) => {
            fs::copy(&local_path, &guest_wasm_path).context("Failed to copy local guest wasm")?;
            println!("Using local guest wasm from {}...", local_path.display());
        }
        ParsedReference::Oci(reference, _) => {
            println!("Pulling guest wasm from {}...", oci_ref);
            let mut client = Client::new(oci_client::client::ClientConfig::default());
            let image_data = client
                .pull(
                    &reference,
                    &oci_client::secrets::RegistryAuth::Anonymous,
                    vec!["application/vnd.wasm.component.v1+wasm", "application/vnd.oci.image.layer.v1.tar+gzip", "application/wasm"],
                )
                .await
                .context("Failed to pull OCI artifact")?;

            if let Some(layer) = image_data.layers.first() {
                fs::write(&guest_wasm_path, &layer.data).context("Failed to write guest wasm")?;
            } else {
                anyhow::bail!("No layers found in the OCI artifact");
            }
        }
    }

    println!("Writing embedded virtualizer...");
    let virtualizer_path = pkg_dir.join("virtualizer.wasm");
    fs::write(&virtualizer_path, VIRTUALIZER_WASM).context("Failed to write virtualizer wasm")?;

    println!("Transpiling virtualizer...");
    let status = Command::new("npx")
        .args(&[
            "-p", "@bytecodealliance/jco@1.26.1",
            "jco", "transpile",
            virtualizer_path.to_str().unwrap(),
            "-o", pkg_dir.join("out-warden").to_str().unwrap(),
            "--async-mode", "jspi",
        ])
        .status()
        .context("Failed to run jco on virtualizer")?;
    
    if !status.success() {
        anyhow::bail!("jco transpile failed for virtualizer");
    }

    println!("Creating module mapper shim...");
    let shim_path = pkg_dir.join("warden_shim.js");
    fs::write(&shim_path, WARDEN_SHIM_JS).context("Failed to write warden shim")?;

    println!("Transpiling guest with mappings...");
    let shim_rel_path = "../warden_shim.js";
    let status = Command::new("npx")
        .args(&[
            "-p", "@bytecodealliance/jco@1.26.1",
            "jco", "transpile",
            guest_wasm_path.to_str().unwrap(),
            "-o", pkg_dir.join("out-guest").to_str().unwrap(),
            "--map", &format!("wasi:cli/environment@0.3.0={}", shim_rel_path),
            "--map", &format!("wasi:filesystem/preopens@0.3.0={}", shim_rel_path),
            "--map", &format!("wasi:filesystem/types@0.3.0={}", shim_rel_path),
            "--map", &format!("wasi:sockets/types@0.3.0={}", shim_rel_path),
            "--map", &format!("wasi:sockets/ip-name-lookup@0.3.0={}", shim_rel_path),
            "--async-mode", "jspi",
        ])
        .status()
        .context("Failed to run jco on guest")?;

    if !status.success() {
        anyhow::bail!("jco transpile failed for guest");
    }

    policy::ensure_policy_exists(&pkg_dir)?;

    println!("Generating Node.js entrypoint...");
    let index_js_path = pkg_dir.join("index.js");
    let index_js_content = r#"import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
process.env.WRDN_POLICY_PATH = path.join(__dirname, 'policy.cedar');

export * from './out-guest/guest.js';
"#;
    fs::write(&index_js_path, index_js_content).context("Failed to write index.js entrypoint")?;

    println!("Installation complete for {}.", oci_ref);
    println!("To use this package, import from '.wrdn/{}/index.js'", pkg_name);
    
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
