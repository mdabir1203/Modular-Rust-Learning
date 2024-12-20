pub fn print_message<T: ?Sized + std::fmt::Display>(message: &T) {
    println!("{}", message);
}

pub fn run_example() {
    let msg_str: &str = "Hello, world!";
    let msg_string: String = String::from("Hello, Rust!");

    print_message(msg_str); // Prints: Hello, world!
    print_message(&msg_string); // Prints: Hello, Rust!

    // Dynamically sized type via Box<dyn Trait>
    let boxed_str: Box<dyn std::fmt::Display> = Box::new("Dynamically Sized Type");
    print_message(&*boxed_str); // Prints: Dynamically Sized Type
}
