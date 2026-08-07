# wrdn

> ⚠️ **Pre-Release Notice**: `wrdn` is currently in early alpha. The WebAssembly Component Model and its surrounding ecosystem are evolving rapidly. Expect breaking changes in future releases as the WASM specifications mature.

`wrdn` is a security tool for Node.js and Deno that allows you to safely run third-party WebAssembly (WASM) components. 

By leveraging the WebAssembly Component Model (WASI 0.3) and Cedar policies, `wrdn` wraps untrusted third-party WASM modules in a secure, memory-safe "Virtualizer". The proxy is written in 100% safe Rust, eliminating memory corruption vectors (`transmute`) commonly found in WASI shims.

## Examples (Supply Chain Scenarios)

The `examples/` directory demonstrates how `wrdn` neutralizes four realistic supply chain attacks using strict Cedar policies:

1. **`01-telemetry-exfiltration`:** A malicious logger attempts to read sensitive environment variables and exfiltrate them via an HTTP POST request. `wrdn` blocks the HTTP request to the unauthorized C2 server.
2. **`02-credential-harvester`:** A rogue image processing library tries to read SSH keys from the filesystem and dump them over a raw TCP socket. `wrdn` restricts filesystem access to designated scratch directories and strictly forbids raw socket connections.
3. **`03-silent-backdoor`:** A compromised data serialization library performs a DNS lookup and attempts to connect a TCP socket to open a reverse shell. `wrdn` intercepts both the DNS query and socket creation at the boundary.
4. **`04-logic-bomb`:** A payload targets production environments by checking `NODE_ENV`. If detected, it attempts to drop a malicious bash script and kill the host process via `wasi:cli/exit`. `wrdn` prevents both unauthorized filesystem writes and premature host exits.

### Exploring the Examples

To run the examples and see `wrdn` intercept the supply chain attacks in real-time, you first need to compile the malicious guest components and generate the `.wrdn` boundaries.

We provide a root `Makefile` to simplify building and testing:
- `make build`: Builds the `wrdn` CLI (`cargo build -p wrdn --release`).
- `make build-guests`: Compiles all the Rust WASM guests in `examples/guests/`.
- `make build-examples`: Runs `wrdn install` for each example to generate the `.wrdn` directories so users can run `node index.js`.
- `make test`: Executes the E2E test suite to verify the policy engine blocks all malicious actions.
- `make clean`: Removes all generated `target/` and `.wrdn/` directories.

To quickly build the examples:
```bash
make build-examples
```

Once the examples are built, simply enter any example directory and run the Node.js host application:
```bash
cd examples/01-telemetry-exfiltration
node index.js
```

## Installation

Run the installation script:

```bash
./install.sh
```

Ensure `~/.cargo/bin` is in your `PATH`.

## Quick Start

You can install a third-party guest WASM component from an OCI registry or a local file:

```bash
wrdn install ghcr.io/org/package:tag
# or
wrdn install file:///path/to/local/component.wasm
```

This command will:
1. Pull the guest WASM component.
2. Transpile the internal `wrdn` virtualizer and the guest component using `jco` into a hidden `.wrdn/{pkg}` directory.
3. Generate a default `policy.cedar` file which **denies all actions** by default.
4. Generate an `index.js` wrapper you can import seamlessly.

### Project Structure
After installation, your directory will look like this:
```
.wrdn/
  {pkg}/
    policy.cedar      <-- The security policy for this specific component
    index.js          <-- The JavaScript wrapper you will import in your app
    guest.wasm        <-- The original untrusted component
    out-guest/        <-- Transpiled JSPI bindings for the guest
    out-warden/       <-- Transpiled JSPI bindings for the Virtualizer
```

### Execution

Import the package in your Node.js application using the generated wrapper:

```javascript
import { telemetry } from './.wrdn/{pkg}/index.js';

// Call functions exactly as defined in the component's WIT interface
await telemetry.runDemo();
```

> **IMPORTANT**: The transpilation relies on JSPI (JavaScript Promise Integration) to seamlessly handle asynchronous execution required by WASI 0.3 capabilities, bridging the synchronous WASM world with Node.js event loops efficiently.
> You **must** run your Node.js application with the following flag:
> 
> ```bash
> node --experimental-wasm-jspi app.js
> ```

## Security & Architecture

`wrdn` enforces a strict **Default-Deny** policy on all actions (including benign ones like `wasi:cli/exit`). To execute, explicit permissions must be given in `policy.cedar`.

Please see [THREAT_MODEL.md](THREAT_MODEL.md) for detailed information on what `wrdn` protects against and its explicit boundaries.

### Performance

wrdn's Cedar policy engine operates extremely efficiently. In preliminary benchmarks (10,000 file reads), the overhead of intercepting and evaluating Cedar rules is virtually imperceptible in Node.js applications, generally introducing less than a microsecond of overhead per action.

## Implementation Status

`wrdn` currently intercepts and enforces security policies on the following WASI 0.3 capabilities:
- `wasi:cli/environment`
- `wasi:filesystem`
- `wasi:sockets`
- `wasi:http`

### WASM Component Composition & Bubbling

You might notice that benign interfaces like `wasi:clocks`, `wasi:random`, and terminal streams (`wasi:cli/stdout`, `wasi:cli/stderr`) are deliberately omitted from the virtualizer's exports. 

This relies on native "bubbling" up to the host via composition tools like `jco` (or `wac`). When a guest component requires a capability that the virtualizer middleware doesn't export, the composition tool automatically bridges that requirement directly to the host runtime.

**Why is this a best practice?**
- **Zero Overhead:** Avoiding WASM-to-WASM context switches for high-traffic or benign streams (like standard output or clocks).
- **Future Proofing:** Omitted capabilities automatically inherit new WASI interface updates without requiring manual code changes in the virtualizer.
- **Security / Threat Model Boundary:** Streams and clocks are generally considered nuisance threats (e.g., terminal spam or UI hangs), not system compromise threats. They belong at the user-agency level, aligning with the Principle of Least Privilege for middleware.

## Policies

The generated `policy.cedar` is intentionally strict. The default behavior is to **Deny All** (`forbid`). You must manually edit this policy to grant specific capabilities to the guest module.

### Action Types and Context
The `wrdn` policy engine intercepts WASI 0.3 capabilities mapped to the following Cedar actions. Each action provides specific context variables you can use in your policy rules (`when { context.<property> == ... }`):

- **Environment & CLI**
  - `Action::"env_read"`: Reading an environment variable. Context: `context.key` (String).
  - `Action::"cli_exit"`: Exiting the process.
  - `Action::"cli_read_environment"`: Accessing the entire environment block.
  - `Action::"cli_read_arguments"`: Accessing the command-line arguments.
  - `Action::"cli_read_initial_cwd"`: Reading the starting directory.

- **Filesystem**
  - `Action::"fs_read"`: Opening/reading a file or directory. Context: `context.path` (String).
  - `Action::"fs_write"`: Modifying or creating a file or directory. Context: `context.path` (String).

- **Networking & Sockets**
  - `Action::"dns_lookup"`: Resolving a domain name. Context: `context.hostname` (String).
  - `Action::"socket_connect"`: Opening a TCP or UDP socket connection. Context: `context.ip` (String), `context.port` (Long).
  
- **HTTP**
  - `Action::"http_outgoing_request"`: Making an outbound HTTP request. Context: `context.url` (String), `context.method` (String).
  - `Action::"http_incoming_request"`: Receiving an HTTP request. Context: `context.url` (String), `context.method` (String).

- **System Resources**
  - `Action::"clock_read_monotonic"`: Reading the monotonic clock.
  - `Action::"clock_read_system"`: Reading the system wall-clock.
  - `Action::"random_read"`: Generating random numbers or seed material.

### Example Policy
To allow the component to read only the `APP_ENV` environment variable, but forbid reading `SECRET_KEY`, you can write:

```cedar
permit(
    principal == User::"guest",
    action == Action::"env_read",
    resource
) when {
    context.key == "APP_ENV"
};
```

When the WebAssembly component attempts to read `APP_ENV`, the virtualizer intercepts the request, evaluates this policy, and returns the value. If it attempts to read `SECRET_KEY`, the virtualizer returns `None` (or throws an error), protecting your host environment.
