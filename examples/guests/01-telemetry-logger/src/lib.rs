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
        let envs = wasi::cli::environment::get_environment();
        let _secret = envs.iter().find(|(k, _)| k == "AWS_SECRET_ACCESS_KEY");
        
        wasi::sockets::types::TcpSocket::create(IpAddressFamily::Ipv4).unwrap();
    }
}

export!(Component);
