# wrdn Examples

This directory contains realistic simulations of modern software supply chain attacks executed via WebAssembly components.

## How to Build and Run

To run these examples, you first need to compile the Rust WASM guest components and use `wrdn install` to generate the `.wrdn` boundary interfaces.

We provide a `Makefile` at the root of the repository to automate this.

**1. Build the Examples**
From the root of the repository, run:
```bash
make build-examples
```
This will compile the Rust guests and install them into the respective example directories.

**2. Run an Example**
Navigate into any example directory and run the Node.js application:
```bash
cd 01-telemetry-exfiltration
node index.js
```

You will see `wrdn` instantly detect and terminate the unauthorized actions (like reading secrets or phoning home) while allowing the application to gracefully crash or continue!

**3. Run the Automated Test Suite**
To verify all examples behave correctly against their policies automatically, run the E2E test script from the root of the repository:
```bash
make test
```
