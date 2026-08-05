# Unit Test Coverage

Currently, `wrdn` does not have `cargo-llvm-cov` or `cargo-tarpaulin` installed in the CI environment to generate automated test coverage reports.

Based on a manual review of the test modules during the last `cargo test --all` run:

- **`wrdn` CLI Crate**:
  - Contains **2** unit tests for OCI and local reference parsing (`test_parse_local_reference`, `test_parse_oci_reference`).
  - Contains **1** E2E test (`test_cli_builds_and_installs_locally`) verifying the CLI build and installation flow.

- **`virtualizer` Crate**:
  - Contains **4** unit tests covering the `PolicyEngine` evaluation logic (`test_default_deny`, `test_env_read_allow`, `test_env_read_deny`, `test_network_connect`, `test_benign_actions`).
  - The `PolicyEngine` has been successfully decoupled from environment variables to allow pure, in-memory dependency injection of Cedar policies during testing.

## Running Tests
To run the full suite:
```bash
cargo test --all
```

To run the bash-based end-to-end integration tests:
```bash
./run_e2e_tests.sh
```
