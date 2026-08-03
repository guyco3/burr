wit_bindgen::generate!({
    world: "telemetry-world",
    path: "wit",
    generate_all
});

use exports::wasi::cli::run::Guest;
use crate::wasi::sockets::types::{IpAddressFamily, IpSocketAddress, Ipv4SocketAddress};

struct Component;

impl Guest for Component {
    async fn run() -> Result<(), ()> {
        let mut results = vec![];

        // 1. Environment
        let envs = crate::wasi::cli::environment::get_environment();
        
        let has_virtual = envs.iter().any(|(k, _)| k == "VIRTUAL");
        results.push(format!("EnvAllow:{}", if has_virtual { "PASS" } else { "FAIL" }));
        
        let has_secret = envs.iter().any(|(k, _)| k == "SECRET_KEY");
        results.push(format!("EnvDeny:{}", if !has_secret { "PASS" } else { "FAIL" }));

        // 2. Filesystem
        let preopens = crate::wasi::filesystem::preopens::get_directories();
        if let Some((dir, _)) = preopens.first() {
            let fs_allow = dir.open_at(crate::wasi::filesystem::types::PathFlags::empty(), "allowed.txt".to_string(), crate::wasi::filesystem::types::OpenFlags::empty(), crate::wasi::filesystem::types::DescriptorFlags::READ).await;
            results.push(format!("FsAllow:{}", if fs_allow.is_ok() { "PASS" } else { "FAIL" }));

            let fs_deny = dir.open_at(crate::wasi::filesystem::types::PathFlags::empty(), "passwd".to_string(), crate::wasi::filesystem::types::OpenFlags::empty(), crate::wasi::filesystem::types::DescriptorFlags::READ).await;
            results.push(format!("FsDeny:{}", if fs_deny.is_err() { "PASS" } else { "FAIL" }));
        } else {
            results.push("FsAllow:FAIL".to_string());
            results.push("FsDeny:FAIL".to_string());
        }

        // 3. DNS
        let net_allow = crate::wasi::sockets::ip_name_lookup::resolve_addresses("api.github.com".to_string()).await;
        results.push(format!("DnsAllow:{}", if net_allow.is_ok() { "PASS" } else { "FAIL" }));

        // 4. TCP
        if let Ok(tcp1) = crate::wasi::sockets::types::TcpSocket::create(IpAddressFamily::Ipv4) {
            let tcp_allow = tcp1.connect(IpSocketAddress::Ipv4(Ipv4SocketAddress { port: 9998, address: (127,0,0,1) })).await;
            results.push(format!("TcpAllow:{}", if tcp_allow.is_ok() { "PASS" } else { "FAIL" }));
        } else {
            results.push("TcpAllow:FAIL".to_string());
        }

        if let Ok(tcp2) = crate::wasi::sockets::types::TcpSocket::create(IpAddressFamily::Ipv4) {
            let tcp_deny = tcp2.connect(IpSocketAddress::Ipv4(Ipv4SocketAddress { port: 9999, address: (127,0,0,1) })).await;
            results.push(format!("TcpDeny:{}", if tcp_deny.is_err() { "PASS" } else { "FAIL" }));
        } else {
            results.push("TcpDeny:FAIL".to_string());
        }

        // 5. UDP
        if let Ok(udp1) = crate::wasi::sockets::types::UdpSocket::create(IpAddressFamily::Ipv4) {
            let udp_allow = udp1.connect(IpSocketAddress::Ipv4(Ipv4SocketAddress { port: 9998, address: (127,0,0,1) }));
            results.push(format!("UdpAllow:{}", if udp_allow.is_ok() { "PASS" } else { "FAIL" }));
        } else {
            results.push("UdpAllow:FAIL".to_string());
        }

        // HTTP tests omitted due to raw FutureReader instantiation complexity in wit-bindgen 0.60.0

        let results_str = results.join(", ");
        println!("Analysis Result: {}", results_str);
        Ok(())
    }
}

export!(Component);
