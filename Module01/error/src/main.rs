use colored::*; // Import the colored crate
use std::io::{self};
use rand::Rng;

/// Represents the possible outcomes of a guess in the number guessing game.
#[derive(Debug, PartialEq)]
enum GuessResult {
    TooSmall,
    TooBig,
    Correct,
}

/// Gets a guess from the user and returns it as a `u32`.
/// 
/// # Returns
/// 
/// A `u32` representing the user's guess if the input is valid, otherwise an error is returned.
fn get_guess() -> Result<u32, String> {
    println!("{}", "Enter your guess: ".blue().bold());
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).map_err(|_| "Failed to read line".to_string())?;
    guess.trim().parse::<u32>().map_err(|_| "Please enter a number".to_string())
}

/// Compares the user's guess to the secret number and returns the result.
/// 
/// # Arguments
/// 
/// * `guess` - The user's guess.
/// * `secret_number` - The secret number to be guessed.
/// 
/// # Returns
/// 
/// A `GuessResult` enum indicating the outcome of the comparison.
fn compare_guess(guess: u32, secret_number: u32) -> GuessResult {
    if guess < secret_number {
        GuessResult::TooSmall
    } else if guess > secret_number {
        GuessResult::TooBig
    } else {
        GuessResult::Correct
    }
}

/// Prints a message based on the result of the guess.
/// 
/// # Arguments
/// 
/// * `result` - The result of the guess comparison.
fn print_guess_result(result: &GuessResult) {
    match result {
        GuessResult::TooSmall => println!("{}", "Too small!".red().bold()),
        GuessResult::TooBig => println!("{}", "Too big!".red().bold()),
        GuessResult::Correct => println!("{}", "You win!".green().bold()),
    }
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        match get_guess() {
            Ok(guess) => {
                println!("You guessed: {}", guess.to_string().green().bold());
                let result = compare_guess(guess, secret_number);
                print_guess_result(&result);
                if result == GuessResult::Correct {
                    break;
                }
            }
            Err(e) => {
                println!("{}", e.red().bold());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_guess() {
        assert_eq!(compare_guess(50, 50), GuessResult::Correct);
        assert_eq!(compare_guess(50, 40), GuessResult::TooBig);
        assert_eq!(compare_guess(50, 60), GuessResult::TooSmall);
    }
}
