wit_bindgen::generate!({
    world: "proxy",
    path: "wit",
    generate_all,
    merge_structurally_equal_types: true
});

pub mod macros;

pub mod cli;
pub mod filesystem;
pub mod http;
pub mod policy;
pub mod sockets;

struct VirtualizationProxy;

export!(VirtualizationProxy);
