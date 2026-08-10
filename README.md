# wrdn

[![E2E Integration](https://github.com/guyco3/wrdn/actions/workflows/integration.yml/badge.svg)](https://github.com/guyco3/wrdn/actions/workflows/integration.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

> ⚠️ **Pre-Release Notice**: `wrdn` is currently in early alpha. Expect breaking changes in future releases as the WebAssembly Component Model (WASI) specifications mature.

`wrdn` is a security middleware for Node.js and Deno that allows you to safely run untrusted third-party WebAssembly (WASM) components. 

By leveraging the WebAssembly Component Model (WASI 0.3) and Cedar policies, `wrdn` wraps untrusted WASM modules in a secure, memory-safe virtualizer proxy written in 100% safe Rust. It strictly intercepts capabilities like filesystem, network, and environment access.

## Installation & Quick Start

You can install the pre-compiled `wrdn` CLI binaries using our install script:
```bash
curl -sL https://raw.githubusercontent.com/guyco3/wrdn/main/install.sh | bash
```

Alternatively, to build the CLI from source, you can clone the repository and run:
```bash
make install
# or: cargo install --path crates/cli
```

You can then install any WebAssembly Component from an OCI registry (like GHCR) or a local file:
```bash
wrdn install ghcr.io/guyco3/parser:0.1.0
```

This generates a `.wrdn` directory containing a secure JavaScript wrapper and a default-deny security policy. You can then import it natively in your Node.js app:
```javascript
import { parser } from './.wrdn/guyco3_parser/index.js';

await parser.parseUppercase("hello world");
```
> **IMPORTANT**: You **must** run your Node.js application with `--experimental-wasm-jspi` (requires Node.js 22+) because WASI 0.3 relies heavily on asynchronous Promises.
> ```bash
> node --experimental-wasm-jspi index.js
> ```

## How it Works

When you install a component with `wrdn`, it performs a transparent architectural maneuver:

1. **The Virtualizer**: It bundles our secure `wrdn` Rust proxy (compiled to `wasm32-wasip2`). This proxy exports the same exact WASI 0.3 interfaces as standard environments (e.g., `wasi:cli/environment`).
2. **The Linkage**: During transpilation (using Bytecode Alliance's `jco`), it strictly maps all of the untrusted Guest's WASI imports into the Virtualizer, rather than out to the Node.js host.
3. **The Sandbox**: The Guest is physically incapable of bypassing the Virtualizer. Every action it takes is evaluated against a dynamic `policy.cedar` ruleset before it is allowed to reach the host system.

If a malicious component tries to access resources it hasn't been explicitly authorized for (such as reading `/etc/passwd`), `wrdn` intercepts and blocks it:
```json
[2026-08-10T06:03:28Z ERROR virtualizer::policy] [WARDEN AUDIT] {"timestamp": 1786341808, "module": "guest-module", "action": "fs_read", "resource": "filesystem", "details": {"path": "/etc/passwd"}, "decision": "DENY"}
```

For a deeper technical dive into the architecture, JSPI, and target compilation, read the [ARCHITECTURE.md](ARCHITECTURE.md) and [THREAT_MODEL.md](THREAT_MODEL.md).

## Security Policies (Cedar)

`wrdn` enforces a strict **Default-Deny** stance. The generated `policy.cedar` file starts completely empty (which evaluates to a strict `forbid`).

To execute any privileged action, explicit permissions must be written in `policy.cedar`. The policy engine uses the Principal ID `guest-module`.

### Supported Actions and Context Variables

| Capability Area | Action (`Warden::Action::...`) | Available Context Variables (`context.<variable>`) | Description |
| :--- | :--- | :--- | :--- |
| **Environment** | `"env_read"` | `key` (String) | Reading an environment variable |
| **Environment** | `"cli_read_environment"` | None | Accessing the entire environment block |
| **Environment** | `"cli_read_arguments"` | None | Accessing the command-line arguments |
| **Environment** | `"cli_read_initial_cwd"` | None | Reading the starting directory |
| **Environment** | `"cli_exit"` | None | Exiting the host process |
| **Filesystem** | `"fs_read"` | `path` (String) | Opening/reading a file or directory |
| **Filesystem** | `"fs_write"` | `path` (String) | Modifying or creating a file or directory |
| **Networking** | `"socket_connect"` | `ip` (String), `port` (Long) | Opening a TCP or UDP socket connection |
| **Networking** | `"socket_bind"` | `ip` (String), `port` (Long) | Binding a raw TCP or UDP socket listener |
| **Networking** | `"dns_lookup"` | `hostname` (String) | Resolving a domain name |
| **HTTP** | `"http_outgoing_request"`| `url` (String), `method` (String) | Making an outbound HTTP request |

### Example Policy

To allow the component to read only the `APP_ENV` environment variable, but forbid reading `SECRET_KEY`, you write:

```cedar
permit(
    principal == Warden::Module::"guest-module",
    action == Warden::Action::"env_read",
    resource
) when {
    context.key == "APP_ENV"
};
```

## Testing & Contributions

Contributions and bug reports are welcome! For detailed instructions on building the project locally, running the test suites, and submitting pull requests, please read our [Contributing Guide](CONTRIBUTING.md).
