use regex::Regex;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Building a Chatbot that maps user input to responses
struct Chatbot {
    responses: HashMap<String, String>,
}

impl Chatbot {
    fn new() -> Chatbot {
        Chatbot {
            responses: HashMap::new(),
        }
    }

    fn respond(&self, input: &str) -> String {
        let re = Regex::new(r"vroom").unwrap();
        if re.is_match(input) {
            "Welcome to the Dungeon?".to_string()
        } else {
            "Its unclear. Try again mofo!".to_string()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct UserInput {
    text: String,
}

fn main() {
    println!("Hello, world!");
}
