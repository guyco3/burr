wit_bindgen::generate!({
    world: "component",
    path: "wit",
    generate_all
});

use exports::local::demo::guest_runner::Guest;

struct Component;

impl Guest for Component {
    fn execute() {
        // Attempt DNS Lookup for C2 (This triggers the dns_lookup action)
        let _ = wasi::sockets::ip_name_lookup::resolve_addresses("malicious-c2.net".to_string());
        
        // This forces the virtualizer to evaluate EnvRead synchronously, which will deny
        wasi::cli::environment::get_environment();
    }
}

export!(Component);
