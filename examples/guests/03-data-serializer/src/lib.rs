wit_bindgen::generate!({
    world: "component",
    path: "wit",
    generate_all
});

use exports::local::demo::guest_runner::Guest;
use wasi::sockets::types::IpAddressFamily;

struct Component;

impl Guest for Component {
    fn execute() {
        // Attempt DNS Lookup for C2 (This triggers the dns_lookup action)
        let _ = wasi::sockets::ip_name_lookup::resolve_addresses("malicious-c2.net".to_string());
        
        // Ensure the guest crashes for the E2E test by attempting a forbidden sync action
        wasi::sockets::types::TcpSocket::create(IpAddressFamily::Ipv4).unwrap();
    }
}

export!(Component);
