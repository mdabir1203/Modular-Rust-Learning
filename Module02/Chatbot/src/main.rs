use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, BufReader};
use serde_json::from_reader;

pub struct Chatbot {
    pub responses: HashMap<String, String>,
}

impl Chatbot {
    pub fn new() -> Chatbot {
        Chatbot {
            responses: HashMap::new(),
        }
    }

    pub fn respond(&self, input: &str) -> String {
        // Use case-insensitive matching and trim input
        let input = input.trim().to_lowercase();

        // Iterate through responses and check for partial matches
        for (key, value) in &self.responses {
            if input.contains(&key.to_lowercase()) {
                return value.clone();
            }
        }

        // Default response if no match is found
        "I'm not sure how to respond to that. Could you please rephrase?".to_string()
    }

    pub fn load_responses(&mut self, filename: &str) -> Result<(), io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let responses: Vec<UserInput> = from_reader(reader)
            .expect("Failed to parse JSON");

        for response in responses {
            self.responses.insert(response.key, response.value);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    pub key: String,
    pub value: String,
}

fn main() -> io::Result<()> {
    let mut chatbot = Chatbot::new();

    // Load responses from JSON file
    if let Err(e) = chatbot.load_responses("chatbot_responses.json") {
        eprintln!("Error loading responses: {}", e);
        return Err(e);
    }

    println!("Chatbot initialized. Type 'exit' to quit.");

    // Starting chatbot loop
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