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
        .args(&["build"])
        .current_dir(&cli_dir)
        .status()
        .expect("Failed to run cargo build for cli");
    
    assert!(status.success(), "CLI build failed");

    // We can also test the local install logic with a dummy file if needed,
    // but the bash script run_e2e_tests.sh covers the full E2E flow.
}
