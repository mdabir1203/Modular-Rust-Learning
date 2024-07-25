use std::io;

fn main() {
    println!("Welcome to crazy cure\n");
    println!("Enter the first number :");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please type a number!");

    println!("Enter the second number :");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please type a number!");

    println!("Choose an operation: +, -, *, /");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read line");

    match operator.trim() {
        "+" => println!("Result: {}", num1 + num2),
        "-" => println!("Result: {}", num1 - num2),
        "*" => println!("Result: {}", num1 * num2),
        "/" => {
            if num2 != 0.0 {
                println!("Result: {}", num1 / num2);
            } else {
                println!("Cannot divide by zero");
            }
        },
        _ => println!("Invalid operation"),
    }
}