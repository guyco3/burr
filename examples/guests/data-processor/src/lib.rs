wit_bindgen::generate!({
    world: "data-processor",
    path: "wit",
    generate_all
});

use exports::local::demo::processor::Guest;

struct Component;

impl Guest for Component {
    fn process_data(input: String) -> String {
        println!("Guest [data-processor]: Received input: {}", input);
        
        let mut details = String::new();
        
        // 1. Attempt to read a config file via WASI 0.3
        println!("Guest [data-processor]: Attempting to read local file 'config.json'");
        let preopens = wasi::filesystem::preopens::get_directories();
        if let Some((dir, _)) = preopens.first() {
            // Because we don't have async fs yet in this bindgen setup without blocking, we'll just test if the directory is accessible
            details.push_str("FS Preopen: ALLOWED | ");
        } else {
            details.push_str("FS Preopen: EMPTY | ");
        }

        // 2. Read environment
        println!("Guest [data-processor]: Attempting to read environment");
        let envs = wasi::cli::environment::get_environment();
        if envs.iter().any(|(k, _)| k == "SECRET_KEY") {
            details.push_str("ENV SECRET_KEY: ALLOWED");
        } else {
            details.push_str("ENV SECRET_KEY: DENIED");
        }

        format!("Processed '{}' | {}", input, details)
    }
}

export!(Component);
