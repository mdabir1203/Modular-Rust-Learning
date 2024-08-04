mod cli;

/// Greets the user with a friendly message.
///
/// This function prints a "Hello, user!" message to the standard output (typically the console).
/// It's designed to demonstrate a simple greeting, potentially as part of a larger CLI application.
/// The function returns a `Result` type, which is a common Rust pattern for error handling.
///
/// # Returns
///
/// - `Ok(())`: Indicates the greeting was displayed successfully without any errors.
/// - `Err(e)`: An error occurred during the operation. The error is wrapped in the `Result` type,
///   allowing the caller to handle it appropriately. This is typically due to issues with I/O operations,
///   such as problems writing to the standard output.

fn greet_user() -> std::io::Result<()> {
    // Hypothetical greeting logic, possibly involving I/O operations
    println!("Hello, misfits and crooks!");
    // If an I/O operation was involved, handle potential errors
    // For demonstration, assume success:
    Ok(())
}


fn main() {
    match cli::greet_user() {
        Ok(_) => println!("Greeting displayed successfully."),
        Err(e) => eprintln!("Failed to display greeting: {}", e),
    }

    match greet_user() {
        Ok(_) => println!("Greeting displayed successfully."),
        Err(e) => eprintln!("Failed to display greeting: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert!(greet_user().is_ok());
    }
}