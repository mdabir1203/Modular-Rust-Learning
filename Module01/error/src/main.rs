use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*; // Import the colored crate

fn main() {
    let secnum = rand::thread_rng().gen_range(1..101);

    loop {
        println!("{}", "Enter your guess: ".blue().bold());
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a number".red().bold());
                continue;
            }
        };

        println!("You guessed: {}", guess.to_string().green().bold());

        match guess.cmp(&secnum) {
            Ordering::Less => println!("{}", "Too small!".red().bold()),
            Ordering::Greater => println!("{}", "Too big!".red().bold()),
            Ordering::Equal => {
                println!("{}", "You win!".green().bold());
                break;
            }
        }
    }
}