wit_bindgen::generate!({
    world: "proxy",
    path: "wit",
    generate_all
});

pub mod cli;
pub mod filesystem;
pub mod http;
pub mod policy;
pub mod sockets;

struct VirtualizationProxy;

export!(VirtualizationProxy);
