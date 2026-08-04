use std::env;
use std::time::SystemTime;

fn main() {
    println!("Telemetry Demo Guest started!");

    // Test environment variables (intercepted by proxy)
    println!("Environment Variables:");
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    // Test clock (intercepted by proxy)
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("Current time: {:?}", n),
        Err(_) => println!("SystemTime before UNIX EPOCH!"),
    }

    println!("Telemetry Demo Guest finished!");
}
