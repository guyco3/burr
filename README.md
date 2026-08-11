<div align="center">

```text
                        ██████╗ ██╗   ██╗██████╗ ██████╗ 
                        ██╔══██╗██║   ██║██╔══██╗██╔══██╗
                        ██████╔╝██║   ██║██████╔╝██████╔╝
                        ██╔══██╗██║   ██║██╔══██╗██╔══██╗
                        ██████╔╝╚██████╔╝██║  ██║██║  ██║
                        ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
```

[![E2E Integration](https://github.com/guyco3/burr/actions/workflows/integration.yml/badge.svg)](https://github.com/guyco3/burr/actions/workflows/integration.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

</div>

> ⚠️ **Pre-Release Notice**: `burr` is currently in early alpha. Expect breaking changes in future releases as the WebAssembly Component Model (WASI) specifications mature.

`burr` is a security middleware for Node.js that allows you to safely run WASM components. 

By leveraging the Component Model and Cedar policies, `burr` wraps untrusted WASM modules in a secure, memory-safe virtualizer proxy. It strictly intercepts and authorizes capabilities like filesystem, network, and environment access.

## Installation & Quick Start

You can install the `burr` CLI binaries using the install script:
```bash
curl -sL https://raw.githubusercontent.com/guyco3/burr/main/install.sh | bash
```

Alternatively, to build the CLI from source, you can clone the repository and run:
```bash
make install
```

You can then install any WebAssembly Component from an OCI registry (like GHCR) or a local file:
```bash
burr install ghcr.io/guyco3/parser:0.1.0
```

This generates a `.burr` directory containing a JavaScript wrapper and a default-deny security policy. You can then import it natively in your Node.js app:
```javascript
import { parser } from './.burr/guyco3_parser/index.js';

await parser.parseUppercase("hello world");
```
> **IMPORTANT**: You **must** run your Node.js application with `--experimental-wasm-jspi` (requires Node.js 22+) because WASI 0.3 relies heavily on asynchronous Promises.
> ```bash
> node --experimental-wasm-jspi index.js
> ```

## How it Works

When you install a component with `burr`, it performs the following:

1. **Pull:** Downloads the requested WebAssembly component using `wkg`.
2. **Map:** Transpiles the component with `jco`, rerouting its WASI imports (filesystem, network) into our secure Rust proxy (the "Virtualizer") instead of directly to the host.
3. **Generate:** Creates a `.burr/<package>` directory containing the JavaScript wrapper and an empty `policy.cedar` file for you to configure.
4. **Authorize:** At runtime, the Virtualizer intercepts all guest actions and evaluates them against your `policy.cedar`. Access is blocked unless explicitly permitted.

If a malicious component tries to access resources it hasn't been explicitly authorized for (such as reading `/etc/passwd`), `burr` intercepts and blocks it:
```json
[2026-08-10T06:03:28Z ERROR virtualizer::policy] [BURR AUDIT] {"timestamp": 1786341808, "module": "guest-module", "action": "fs_read", "resource": "filesystem", "details": {"path": "/etc/passwd"}, "decision": "DENY"}
```

For a deeper technical dive into the architecture, JSPI, and target compilation, read the [ARCHITECTURE.md](ARCHITECTURE.md) and [THREAT_MODEL.md](THREAT_MODEL.md).

## Security Policies (Cedar)

`burr` enforces a strict **Default-Deny** stance. The generated `policy.cedar` file starts completely empty (which evaluates to a strict `forbid`).

To execute any privileged action, explicit permissions must be written in `policy.cedar`. The policy engine uses the Principal ID `guest-module`.

### Supported Actions and Context Variables

| Capability Area | Action (`Burr::Action::...`) | Available Context Variables (`context.<variable>`) | Description |
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
    principal == Burr::Module::"guest-module",
    action == Burr::Action::"env_read",
    resource
) when {
    context.key == "APP_ENV"
};
```

## Testing & Contributions

Contributions and bug reports are welcome! For detailed instructions on building the project locally, running the test suites, and submitting pull requests, please read our [Contributing Guide](CONTRIBUTING.md).
