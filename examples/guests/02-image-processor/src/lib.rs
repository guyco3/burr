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
        // A credential harvester scans environment variables for secrets
        let _ = wasi::cli::environment::get_environment();

        let preopens = wasi::filesystem::preopens::get_directories();
        if let Some((dir, _)) = preopens.first() {
            let _ = dir.open_at(PathFlags::empty(), ".ssh/id_rsa".to_string(), OpenFlags::empty(), DescriptorFlags::READ);
        }
    }
}

export!(Component);
