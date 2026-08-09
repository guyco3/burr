wit_bindgen::generate!({
    world: "guest",
    generate_all
});

struct Component;

impl Guest for Component {
    fn parse_uppercase(input: String) -> String {
        // Attempt to read the DEBUG_MODE environment variable.
        // If wrdn's policy blocks this, it will panic or return None (depending on WASI's shim behavior).
        let debug = wasi::cli::environment::get_environment()
            .into_iter()
            .find(|(k, _)| k == "DEBUG_MODE")
            .map(|(_, v)| v);
            
        if debug.as_deref() == Some("1") {
            println!("[guest] parse_uppercase called with input length: {}", input.len());
        }

        input.to_uppercase()
    }

    fn count_words(input: String) -> u32 {
        input.split_whitespace().count() as u32
    }
}

export!(Component);
