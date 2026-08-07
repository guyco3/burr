# Threat Model

wrdn secures the host from the guest. It does not protect against memory corruption vulnerabilities internal to the WASM guest itself.

## Architecture

wrdn uses the WASM Component Model to establish a strict boundary between the guest module and the host system. All capabilities (like filesystem and network access) requested by the guest are intercepted by the wrdn virtualizer proxy.

## Trust Boundaries

- **The Host (Trusted):** The underlying system running the Node.js/Deno runtime. wrdn guarantees that guest modules cannot access host resources unless explicitly permitted by the Cedar policy.
- **The Policy (Trusted):** The `policy.cedar` file dictates exactly what actions are allowed. The engine enforces a strict **default-deny** stance.
- **The Guest (Untrusted):** The WebAssembly module being executed. We assume the guest is potentially malicious or compromised.

## Out of Scope

- **Guest Memory Safety:** If the guest module contains a buffer overflow or logic bug that allows an attacker to manipulate the guest's own memory, wrdn cannot prevent this. wrdn only restricts the *capabilities* the guest has access to on the host.
- **Denial of Service (CPU/Memory):** wrdn currently does not enforce CPU or memory quotas on the WASM execution.
