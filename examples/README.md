# wrdn Examples

This directory contains examples demonstrating how to use `wrdn` to secure WebAssembly components in Node.js applications.

## Directory Structure

*   **`guests/`**: Contains the Rust source code for the WebAssembly guests used in the examples.
    *   **`telemetry-demo/`**: A standard command/CLI component that collects system telemetry (environment variables, etc.) and demonstrates WASI 0.3 interception.
    *   **`data-processor/`**: A "reactor" component (a long-running service/library) that exports a `process-data` function and tests WASI 0.3 capabilities from an interface, demonstrating that `wrdn` works seamlessly with Reactor Components as well.
*   **`01-telemetry-allow-all/`**: Demonstrates running `telemetry_demo.wasm` with an "Allow All" Cedar policy.
*   **`02-telemetry-deny-all/`**: Demonstrates running the same telemetry component, but with a default "Deny All" policy to see how the Warden gracefully blocks unauthorized capability requests (e.g., denying `env_read`).
*   **`03-telemetry-granular/`**: Demonstrates running the telemetry component with a highly granular policy that explicitly allows certain environment variables (like `APP_ENV`) while denying others.
*   **`04-reactor-data-processor/`**: Demonstrates running a Reactor Component (`data_processor.wasm`) that exports functions like `processData()`, interacting with it programmatically from Node.js, and using a granular policy that specifically permits reading `SECRET_KEY`.

## Running the Examples

Ensure you have built the CLI (`wrdn`) and that it is in your `PATH` or available locally. Alternatively, the examples can be run sequentially via the end-to-end test script in the repository root.

To run a specific example manually:

1.  **Build the Guest WebAssembly modules:**
    ```bash
    cd guests/telemetry-demo
    cargo build --target=wasm32-wasip2 --release
    
    cd ../data-processor
    cargo build --target=wasm32-wasip2 --release
    ```

2.  **Navigate to an example scenario:**
    ```bash
    cd ../../04-reactor-data-processor
    ```

3.  **Install the component with `wrdn`:**
    ```bash
    # Assuming you built the CLI and the WASM guest
    cargo run -p wrdn -- install "file://$(pwd)/../guests/data-processor/target/wasm32-wasip2/release/data_processor.wasm"
    ```
    *(Note: This creates a `.wrdn/` directory containing the component, the virtualizer, and a `policy.cedar` file)*

4.  **Run the Node.js Host:**
    ```bash
    # Execute the host code with WebAssembly JSPI enabled
    node --experimental-wasm-jspi index.js
    ```

5.  **Review the Output:**
    You should see the output of the component's execution, along with `[WARDEN AUDIT]` logs dynamically showing `ALLOW` and `DENY` decisions made by the Cedar policy engine at runtime.
