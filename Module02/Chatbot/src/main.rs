use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Write};

/**
 * @file main.rs
 * @brief Rusty: A simple chatbot implemented in Rust.
 */

pub struct Chatbot {
    responses: HashMap<String, String>,
}

impl Chatbot {
    pub fn new() -> Self {
        Chatbot {
            responses: HashMap::new(),
        }
    }

    pub fn respond(&self, input: &str) -> String {
        let input = input.trim().to_lowercase();

        for (key, value) in &self.responses {
            if input.contains(&key.to_lowercase()) {
                return value.clone();
            }
        }

        String::from("I'm not sure how to respond to that. Could you please rephrase?")
    }

    pub fn load_responses(&mut self, filename: &str) -> Result<(), io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let responses: Vec<UserInput> = serde_json::from_reader(reader)?;

        for response in responses {
            self.responses.insert(response.key, response.value);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    key: String,
    value: String,
}

fn main() -> io::Result<()> {
    let mut chatbot = Chatbot::new();

    if let Err(e) = chatbot.load_responses("chatbot_responses.json") {
        eprintln!("Error loading responses: {}", e);
        return Err(e);
    }

    println!("Chatbot initialized. Type 'exit' to quit.");

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        let response = chatbot.respond(input);
        println!("Bot: {}", response);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chatbot_respond() {
        let mut chatbot = Chatbot::new();
        chatbot
            .responses
            .insert("hello".to_string(), "Hi there!".to_string());
        chatbot
            .responses
            .insert("bye".to_string(), "Goodbye!".to_string());

        assert_eq!(chatbot.respond("Hello"), "Hi there!");
        assert_eq!(chatbot.respond("Goodbye"), "Goodbye!");
        assert_eq!(
            chatbot.respond("How are you?"),
            "I'm not sure how to respond to that. Could you please rephrase?"
        );
    }

    #[test]
    fn test_chatbot_load_responses() {
        let mut chatbot = Chatbot::new();
        chatbot.load_responses("chatbot_responses.json").unwrap();

        assert_eq!(chatbot.responses.len(), 4);
        assert_eq!(
            chatbot.responses.get("hello").unwrap(),
            "Hi there! How can I help you?"
        );
        assert_eq!(
            chatbot.responses.get("bye").unwrap(),
            "Goodbye! Have a great day!"
        );
        assert_eq!(
            chatbot.responses.get("how are you").unwrap(),
            "I'm a bot, so I don't have feelings, but thanks for asking!"
        );
        assert_eq!(
            chatbot.responses.get("Who are you").unwrap(),
            "I'm YOU, you MOFO!!"
        );
    }
}
