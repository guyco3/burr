use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_cli_builds_and_installs_locally() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cli_dir = PathBuf::from(&manifest_dir);

    // Test that the CLI builds successfully (this implicitly tests build.rs logic)
    let status = Command::new("cargo")
        .args(["build"])
        .current_dir(&cli_dir)
        .status()
        .expect("Failed to run cargo build for cli");

    assert!(status.success(), "CLI build failed");

    // Test local install logic
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let dummy_wasm_path = temp_dir.path().join("dummy.wasm");
    // A valid minimal WebAssembly Component (version 1)
    fs::write(&dummy_wasm_path, b"\x00asm\x0d\x00\x01\x00").expect("Failed to write dummy wasm");

    let bin_path = env!("CARGO_BIN_EXE_burr");
    let status = Command::new(bin_path)
        .args(["install", &format!("file://{}", dummy_wasm_path.display())])
        .current_dir(temp_dir.path())
        .status()
        .expect("Failed to run burr install");

    // Ensure the CLI exited successfully
    assert!(status.success(), "CLI install failed");

    // Verify expected output artifacts
    let burr_dir = temp_dir.path().join(".burr/dummy");
    assert!(burr_dir.exists(), ".burr directory was not created");
    assert!(burr_dir.join("guest.wasm").exists(), "guest.wasm missing");
    assert!(
        burr_dir.join("virtualizer.wasm").exists(),
        "virtualizer.wasm missing"
    );
    assert!(
        burr_dir.join("burr_shim.js").exists(),
        "burr_shim.js missing"
    );
    let policies_dir = temp_dir.path().join("policies");
    assert!(
        policies_dir.join("dummy_policy.cedar").exists(),
        "dummy_policy.cedar missing"
    );
    assert!(burr_dir.join("index.js").exists(), "index.js missing");

    let manifest_path = temp_dir.path().join("burr.json");
    assert!(manifest_path.exists(), "burr.json missing");
    let manifest_content = fs::read_to_string(&manifest_path).unwrap();
    assert!(manifest_content.contains("\"dummy\": \"file://"));

    // Note: The jco transpilation might fail if it tries to parse the dummy wasm as a valid component,
    // but our main goal is to test the filesystem interactions and CLI orchestration.
}
