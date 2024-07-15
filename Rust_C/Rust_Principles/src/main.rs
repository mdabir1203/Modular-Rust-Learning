mod cli;

pub fn greet_user() -> std::io::Result<()> {
    // Hypothetical greeting logic, possibly involving I/O operations
    println!("Hello, user!");
    // If an I/O operation was involved, handle potential errors
    // For demonstration, assume success:
    Ok(())
}

fn main() {
    match cli::greet_user() {
        Ok(_) => println!("Greeting displayed successfully."),
        Err(e) => eprintln!("Failed to display greeting: {}", e),
    }
}
