# Burr Repository Agent Context

This repository (`burr`) contains a security middleware proxy for Node.js that enables running untrusted WebAssembly Component Model (WASI) modules safely. 

## Core Architecture
- **Virtualizer**: A Rust-based WebAssembly Component (compiled to `wasm32-wasip2`) that intercepts WASI 0.3 imports.
- **jco Transpilation**: The CLI fetches a guest component and uses `jco` to transpile it, statically mapping its WASI imports (e.g. `wasi:filesystem`, `wasi:sockets`) to the `burr` virtualizer instead of the Node.js host.
- **Cedar Policies**: The virtualizer uses the Cedar policy engine to evaluate all intercepted actions against a default-deny policy.

## Directory Structure
When a user runs `burr install <package>`:
1. **`.burr/` (Hidden/Git-ignored)**: Contains the transpiled JS wrappers and the WASM binaries (`guest.wasm`, `virtualizer.wasm`).
2. **`policies/` (Visible/Committed)**: Contains the `<package>_policy.cedar` file, which is the Cedar security configuration that users edit.

## ESM Initialization Order
To ensure the WASI environment correctly loads the policy via environment variables, the system uses a discrete `setup.js` module. `setup.js` resolves the policy path dynamically (navigating up from `.burr/<pkg>` to `policies/<pkg>_policy.cedar`) and reads it into `process.env.BURR_POLICY_CONTENT` *before* the main WASM virtualizer initializes.

## Development & Testing
- **Dependencies**: The project relies on Node.js 22+ with `--experimental-wasm-jspi`.
- **Testing**: `make test` runs both unit tests and a Docker-based integration test suite (`tests/integration/run_tests.sh`). The integration tests overwrite the generated `policies/<pkg>_policy.cedar` to verify the boundary holds.

## Reference Material
- **WASI 0.3**: Guest components must import WASI 0.3+ interfaces. WASI 0.2 `pollable` streams are incompatible with JSPI async.
- **Bytecode Alliance**: Refer to [Creating Runnable Components in Rust](https://component-model.bytecodealliance.org/language-support/creating-runnable-components/rust.html) for `wasm32-wasip2` and `wit-bindgen` details.
