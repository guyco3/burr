wit_bindgen::generate!({
    world: "component",
    path: "wit",
    generate_all
});

use exports::local::demo::guest_runner::Guest;
use wasi::filesystem::types::{PathFlags, OpenFlags, DescriptorFlags};
use wasi::sockets::types::IpAddressFamily;

struct Component;

impl Guest for Component {
    fn execute() {
        let preopens = wasi::filesystem::preopens::get_directories();
        if let Some((dir, _)) = preopens.first() {
            let _ = dir.open_at(PathFlags::empty(), ".ssh/id_rsa".to_string(), OpenFlags::empty(), DescriptorFlags::READ);
            wasi::sockets::types::TcpSocket::create(IpAddressFamily::Ipv4).unwrap();
        }
    }
}

export!(Component);
