use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let virtualizer_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .join("virtualizer");

    println!("cargo:rerun-if-changed={}", virtualizer_dir.display());

    let target_dir = out_dir.clone() + "/virt_target";
    let status = Command::new("cargo")
        .args(&["build", "--target=wasm32-wasip2", "--release"])
        .env("CARGO_TARGET_DIR", &target_dir)
        .current_dir(&virtualizer_dir)
        .status()
        .expect("Failed to build virtualizer crate");

    if !status.success() {
        panic!("Failed to build virtualizer crate. Ensure you have the WASM target installed: `rustup target add wasm32-wasip2`");
    }

    let wasm_file = PathBuf::from(&target_dir)
        .join("wasm32-wasip2")
        .join("release")
        .join("virtualizer.wasm");

    let dest_path = PathBuf::from(&out_dir).join("virtualizer.wasm");
    fs::copy(&wasm_file, &dest_path).expect("Failed to copy virtualizer.wasm to OUT_DIR");
}
