# wrdn

`wrdn` is a tool to safely run third-party code as WASM modules in Node environments using `jco` and WASI 0.3. It operates by adding a virtualizer WASM module that intercepts guest module actions and validates them against a policy file.

## Installation

Run the installation script:

```bash
./install.sh
```

Ensure `~/.cargo/bin` is in your `PATH`.

## Usage

You can install a third-party guest WASM component from an OCI registry:

```bash
wrdn install ghcr.io/org/package:tag
```

This command will:
1. Pull the guest WASM component.
2. Transpile the internal `wrdn` virtualizer and the guest component using `jco` into a hidden `.wrdn/{pkg}` directory.
3. Generate a default `policy.cedar` file which **denies all actions** by default.

### Execution

> **IMPORTANT**: The transpilation relies on JSPI (JavaScript Promise Integration).
> This means `wrdn` currently only works on V8-based JavaScript engines (like Node.js and Deno).
> 
> You **must** run your application with the following flag:
> 
> ```bash
> node --experimental-wasm-jspi app.js
> ```
> 
> We recommend adding this flag to your `package.json` scripts.

## Policies

The generated `policy.cedar` is intentionally strict (`forbid`). You must manually edit this policy to grant specific capabilities to the guest module.
