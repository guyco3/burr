wit_bindgen::generate!({
    world: "guest",
    generate_all
});

struct Component;

impl Guest for Component {
    fn parse_uppercase(input: String) -> String {
        let secret = wasi::cli::environment::get_environment()
            .into_iter()
            .find(|(k, _)| k == "TUTORIAL_SECRET")
            .map(|(_, v)| v);
            
        if secret.as_deref() == Some("1") {
            println!("You found me!");
        }

        input.to_uppercase()
    }

    fn count_words(input: String) -> u32 {
        input.split_whitespace().count() as u32
    }
}

export!(Component);
