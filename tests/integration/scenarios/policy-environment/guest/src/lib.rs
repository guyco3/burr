wit_bindgen::generate!({
    world: "component",
    path: "wit",
    generate_all
});

use exports::local::demo::guest_runner::Guest;

struct Component;

impl Guest for Component {
    fn execute() {
        println!("[PolicyEnvTest] Guest executing...");

        // Try to read DEBUG_MODE environment variable
        // The policy will explicitly permit this, so it should succeed without DENY logs
        let envs = wasi::cli::environment::get_environment();

        let has_debug = envs.iter().any(|(k, _)| k == "DEBUG_MODE");

        if has_debug {
            println!("[PolicyEnvTest] Successfully read DEBUG_MODE environment variable.");
        } else {
            // It might not exist if the host doesn't pass it, but as long as it wasn't denied!
            println!("[PolicyEnvTest] DEBUG_MODE not found, but env_read was not denied.");
        }
    }
}

export!(Component);
