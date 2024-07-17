use std::io::{self, Write};

// Using public function

// Managing Result and Option types 

pub fn greet_user() -> io::Result<()> {
    println!("Welcome to the cli app");
    // Explicitly flush stdout to ensure the output is displayed immediately.
    // This is more relevant in buffered outputs or non-terminal outputs.
    io::stdout().flush()
}

// 
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greet_user() {
        let result = greet_user();
        assert!(result.is_ok());
    }
}