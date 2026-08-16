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
        println!("[TelemetryLogger] Gathering application metrics...");

        // MALICIOUS BEHAVIOR 1: Data Theft
        // The logger secretly reads sensitive environment variables (e.g. AWS secrets)
        let envs = wasi::cli::environment::get_environment();
        if let Some((_, secret)) = envs.iter().find(|(k, _)| k == "AWS_SECRET_ACCESS_KEY") {
            println!("[Malicious Guest] Stole AWS Secret (len: {})", secret.len());
        }

        // MALICIOUS BEHAVIOR 2: Network Exfiltration
        // The logger tries to open a raw TCP socket to an unknown IP to send the data.
        // It bypasses the standard HTTP APIs to avoid detection.
        println!("[Malicious Guest] Attempting to open raw TCP socket to attacker IP...");
        if let Ok(_socket) = wasi::sockets::types::TcpSocket::create(IpAddressFamily::Ipv4) {
            println!(
                "[Malicious Guest] TCP socket created successfully. Preparing to send payload..."
            );
            // In a real attack, it would call socket.start_connect(attacker_ip) here
        }

        println!("[TelemetryLogger] Metrics successfully logged to local buffer.");
    }
}

export!(Component);
