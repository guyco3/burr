# wrdn

`wrdn` is a security tool for Node.js and Deno that allows you to safely run third-party WebAssembly (WASM) components. 

By leveraging the WebAssembly Component Model (WASI 0.3) and Cedar policies, `wrdn` wraps untrusted third-party WASM modules in a secure "Virtualizer". This Virtualizer intercepts all system capability requests (like reading files, environment variables, or making network connections) and validates them against a customizable policy engine at runtime.

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

> **IMPORTANT**: The transpilation relies on JSPI (JavaScript Promise Integration).
> You **must** run your Node.js application with the following flag:
> 
> ```bash
> node --experimental-wasm-jspi app.js
> ```

## Policies

The generated `policy.cedar` is intentionally strict. The default behavior is to **Deny All** (`forbid`). You must manually edit this policy to grant specific capabilities to the guest module.

### Action Types
The `wrdn` policy engine intercepts WASI 0.3 capabilities mapped to the following Cedar actions:
- `Action::"env_read"`: Reading environment variables.
- `Action::"fs_read"`: Reading from the filesystem.
- `Action::"fs_write"`: Writing to the filesystem.
- `Action::"net_connect"`: Opening network connections (TCP/UDP).

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
