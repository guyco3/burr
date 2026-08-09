# Tutorial: Using wrdn with GitHub Container Registry

This tutorial demonstrates how to install a WebAssembly Component from an OCI registry (GitHub Container Registry) and protect your Node.js application from its hidden behaviors using `wrdn`.

## The Scenario

We have built a simple WASM component named `parser`. It exports two functions:
1. `count-words(input)`
2. `parse-uppercase(input)`

However, the author of this component included a hidden behavior: when `parse-uppercase` is called, it attempts to read the `DEBUG_MODE` environment variable from the host system. By default, `wrdn`'s strict Default-Deny policy will block this!

---

## Step 1: Install the Component

In your terminal, navigate to the `app/` directory of this tutorial:
```bash
cd app
```

Install the guest module directly from the public GitHub Container Registry:
```bash
wrdn install ghcr.io/guyco3/parser:0.1.0
```

This command downloads the component and automatically wraps it in the `wrdn` secure virtualizer. You will see a new `.wrdn/` directory created.

## Step 2: Run the App (Blocked by Policy)

The `index.js` file imports the protected component and calls both of its functions. 

Run the application:
```bash
npm start
# or: node --experimental-wasm-jspi index.js
```

**Expected Output:**
You should see that `count-words` succeeds, but `parse-uppercase` throws an error. This is because `wrdn` intercepted the component's attempt to read the environment variable, and the default policy (`.wrdn/guyco3_parser/policy.cedar`) denies all actions.

## Step 3: Edit the Policy

To allow the component to execute successfully, we must explicitly permit it to read the `DEBUG_MODE` environment variable.

Open `.wrdn/guyco3_parser/policy.cedar` and add the following Cedar rule:

```cedar
permit(
    principal == Warden::Module::"guest-module",
    action == Warden::Action::"env_read",
    resource
) when {
    context.key == "DEBUG_MODE"
};
```

## Step 4: Run Again (Success)

Run the application again:
```bash
npm start
```

**Expected Output:**
The application should now complete successfully! The Virtualizer intercepted the request, evaluated your new policy, saw that `DEBUG_MODE` was explicitly allowed, and permitted the WebAssembly component to read it.

---

## (Optional) How the Component was Published

If you want to build and publish your own components to GHCR, here are the exact commands we used to publish this tutorial's component:

```bash
# Navigate to the guest source code
cd guest/

# Compile the Rust code into a WASI 0.3 component
cargo build --target wasm32-wasip2

# Publish to the registry using wkg (WasmPkg)
# Note: You must be authenticated via `docker login ghcr.io` first!
wkg oci push ghcr.io/your-username/parser:0.1.0 ../../../target/wasm32-wasip2/debug/parser.wasm
```
