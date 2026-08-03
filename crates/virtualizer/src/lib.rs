wit_bindgen::generate!({
    world: "proxy",
    path: "wit",
    generate_all
});

pub mod policy;
pub mod filesystem;
pub mod sockets;
pub mod cli;
pub mod clocks;
pub mod random;

struct VirtualizationProxy;

export!(VirtualizationProxy);

