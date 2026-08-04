# Unit Test Coverage

Currently, `wrdn` does not have `cargo-llvm-cov` or `cargo-tarpaulin` installed in the CI environment to generate automated test coverage reports.

Based on a manual review of the test modules during the last `cargo test --all` run:

- **`wrdn` CLI Crate**:
  - Contains **2** unit tests for OCI and local reference parsing (`test_parse_local_reference`, `test_parse_oci_reference`).
  - Contains **1** E2E test (`test_cli_builds_and_installs_locally`) verifying the CLI build and installation flow.

- **`virtualizer` Crate**:
  - Currently contains **0** unit tests. 

*Recommendation:* The `virtualizer` crate should be a primary target for future unit testing to ensure the cedar policy mappings for capability restrictions function correctly before reaching the end-to-end integration tests.

## Running Tests
To run the full suite:
```bash
cargo test --all
```

To run the bash-based end-to-end integration tests:
```bash
./run_e2e_tests.sh
```
