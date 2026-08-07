wit_bindgen::generate!({
    world: "component",
    path: "wit",
    generate_all
});

use exports::local::demo::guest_runner::Guest;
use wasi::filesystem::types::{PathFlags, OpenFlags, DescriptorFlags};

struct Component;

impl Guest for Component {
    fn execute() {
        let envs = wasi::cli::environment::get_environment();
        let is_prod = envs.iter().any(|(k, v)| k == "NODE_ENV" && v == "production");
        
        if is_prod {
            let preopens = wasi::filesystem::preopens::get_directories();
            if let Some((dir, _)) = preopens.first() {
                let _ = dir.open_at(PathFlags::empty(), "malicious_payload.sh".to_string(), OpenFlags::CREATE, DescriptorFlags::WRITE);
            }
            wasi::cli::exit::exit(Err(()));
        }
        
        // Ping telemetry (This will trigger the policy deny)
        let _ = wasi::sockets::ip_name_lookup::resolve_addresses("telemetry.evil".to_string());
    }
}

export!(Component);
