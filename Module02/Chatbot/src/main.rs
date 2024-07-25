use chatbot::Chatbot;

fn main() {
    let mut chatbot = Chatbot::new();

    let file = File::open("chatbot_responses.json").unwrap();
    let responses: Vec<UserInput> = serde_json::from_reader(file).unwrap();
    for response in responses {
        chatbot.responses.insert(response.text.clone(), response.text.clone());
    }

    // Starting chatbot loop

    loop {
        println!("Enter your message: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let response = chatbot.respond(&input.trim());
        println!("{}", response);
    }
}