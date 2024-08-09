use std::io::{self, Write};

/// Greets the user with a friendly message.
///
/// This function prints a "Welcome to the CLI app" message to the standard output (typically the console).
/// It's designed to demonstrate a simple greeting, potentially as part of a larger CLI application.
/// The function returns a `Result` type, which is a common Rust pattern for error handling.
///
/// # Returns
///
/// - `Ok(())`: Indicates the greeting was displayed successfully without any errors.
/// - `Err(e)`: An error occurred during the operation. The error is wrapped in the `Result` type,
///   allowing the caller to handle it appropriately. This is typically due to issues with I/O operations,
///   such as problems writing to the standard output.
pub fn greet_user() -> io::Result<()> {
    println!("Welcome to the CLI app");
    io::stdout().flush()?;
    Ok(())
}

#[cfg(test)]
/// Test suite for the `greet_user` function.
mod tests {
    use super::*;

    /// Test case to verify that the `greet_user` function returns a result without any errors.
    #[test]
    fn test_greet_user() {
        assert!(greet_user().is_ok());
    }
}
