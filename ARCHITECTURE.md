# Architecture Overview

`wrdn` is designed to provide a secure execution environment for WebAssembly Component Model modules by acting as an intercepting proxy. It leverages WASI 0.3 interfaces to virtualize system capabilities.

## Compilation Workflow

The workflow relies on a specific combination of tools and targets to ensure that the Component Model works correctly within JavaScript environments.

### 1. The Targets and Bindings
Both the internal `virtualizer` and the third-party guest components are compiled using the `wasm32-wasip2` target.
- **`wasm32-wasip2`**: This Rust target produces Core WebAssembly that adheres to the WebAssembly System Interface (WASI) Preview 2/3 ABI. 
- **`wit-bindgen` (v0.60.0+)**: Used to generate Rust bindings from `.wit` files. It translates high-level Component Model interfaces (like `wasi:cli/environment`) into Rust traits and structures. Version 0.60.0+ is strictly required to support the `async` functions and stream resources introduced in WASI 0.3.

### 2. The `wrdn` Virtualization Strategy
When you run `wrdn install`, the CLI performs the following steps:
1. **Fetch**: Downloads the `.wasm` component.
2. **Virtualizer Injection**: The CLI contains a bundled `virtualizer.wasm` module. This module exports the same WASI 0.3 interfaces that standard environments provide (e.g., `wasi:cli/environment`, `wasi:filesystem/preopens`), but implements them internally using the Cedar Policy Engine.
3. **Transpilation (`jco`)**: 
   - Uses `@bytecodealliance/jco` (v1.26.1).
   - The Virtualizer is transpiled first, mapping its host requirements to the Node.js native host environment.
   - The Guest component is transpiled second. Crucially, the CLI instructs `jco` to **map the Guest's WASI imports to the transpiled Virtualizer**.
   - **Result**: The Guest believes it is talking to the host OS, but it is actually talking to the Virtualizer.

### 3. JavaScript Promise Integration (JSPI)
Because WASI 0.3 utilizes `async func` heavily for non-blocking I/O, the resulting JavaScript bindings rely on WebAssembly JSPI. This allows WebAssembly functions to suspend execution, wait for a JavaScript Promise (like reading a file or evaluating a Cedar policy), and resume seamlessly. 

This requires the `--experimental-wasm-jspi` flag in Node.js.

## WASI 0.3 to WASI 0.2 Polyfilling
While the Virtualizer and Guest are written using **WASI 0.3** concepts (like native async and streams), the current ecosystem (including `jco` and V8) natively targets **WASI 0.2** runtime semantics. 

`wit-bindgen` and `jco` work together to polyfill WASI 0.3 interfaces back down to WASI 0.2 `pollable` concepts under the hood. This ensures that we can write clean, asynchronous Rust code in the virtualizer, while still executing correctly on modern JS engines.
