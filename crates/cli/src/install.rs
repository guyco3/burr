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

pub async fn run_install(oci_ref: &str) -> Result<()> {
    let pkg_name;
    let guest_wasm_path;
    let pkg_dir;
    
    if let Some(local_path) = oci_ref.strip_prefix("file://") {
        pkg_name = Path::new(local_path).file_stem().unwrap().to_str().unwrap().to_string();
        let wrdn_dir = PathBuf::from(".wrdn");
        pkg_dir = wrdn_dir.join(&pkg_name);
        fs::create_dir_all(&pkg_dir).context("Failed to create .wrdn pkg directory")?;
        
        guest_wasm_path = pkg_dir.join("guest.wasm");
        fs::copy(local_path, &guest_wasm_path).context("Failed to copy local guest wasm")?;
        println!("Using local guest wasm from {}...", local_path);
    } else {
        let reference: Reference = oci_ref.parse().context("Invalid OCI reference")?;
        pkg_name = reference.repository().replace("/", "_");
        
        let wrdn_dir = PathBuf::from(".wrdn");
        pkg_dir = wrdn_dir.join(&pkg_name);
        fs::create_dir_all(&pkg_dir).context("Failed to create .wrdn pkg directory")?;

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

        guest_wasm_path = pkg_dir.join("guest.wasm");
        if let Some(layer) = image_data.layers.first() {
            fs::write(&guest_wasm_path, &layer.data).context("Failed to write guest wasm")?;
        } else {
            anyhow::bail!("No layers found in the OCI artifact");
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

    println!("Installation complete for {}.", oci_ref);
    println!("To run, use 'node --experimental-wasm-jspi ...'");

    Ok(())
}
