# Simple Calculator

The Crazy Cure Calculator is a simple, interactive command-line application written in Rust. It enables users to perform basic arithmetic operations, including addition, subtraction, multiplication, and division, on two input numbers. This document provides a technical and user-friendly overview of the application's functionality.

## Features

- User Interaction: Prompts users for input through the command line.
- Supports Basic Arithmetic Operations: Users can perform addition (+), subtraction (-), multiplication (*), and division (/) on two numbers.
- Error Handling: The application gracefully handles invalid inputs by prompting the user to enter a valid number.

## Getting Started

### Prerequisites

- Rust programming language setup on your machine. Visit [The Rust Programming Language](https://www.rust-lang.org/) website for installation instructions.

### Running the Application

1. Clone the Repository: First, clone the repository to your local machine using Git.
2. Navigate to the Application Directory: Change into the directory containing the application.
3. Run the Application: Use Cargo, Rust's package manager and build system, to compile and run the application.
4. Follow the On-Screen Prompts: The application will guide you through entering two numbers and selecting an operation to perform.

## How It Works

### Technical Details

- Reading Input: The application uses Rust's `std::io` library to read user input from the command line.
- Parsing Input: Input numbers are initially read as strings and then parsed into 64-bit floating-point numbers (`f64`) for arithmetic operations.
- Selecting Operations: A `match` statement is used to determine the operation based on the user's choice, performing the corresponding arithmetic operation on the input numbers.
- Error Handling: The application checks for valid numerical input and responds with appropriate error messages if the input is invalid.

### User Interaction Flow

1. Welcome Message: Upon starting, the application displays a welcome message.
2. Input Numbers: Users are prompted to enter two numbers, one at a time.
3. Select Operation: Users choose an arithmetic operation to perform on the entered numbers.
4. Display Result: The application calculates and displays the result of the chosen operation.

## Conclusion

The Simple Calculator is a straightforward yet powerful example of how Rust can be used to create interactive command-line applications. It demonstrates basic programming concepts such as input/output operations, parsing strings to numbers, conditional logic with `match` statements, and error handling.

For further exploration, users can extend the application to include more complex operations, improve error handling, or enhance the user interface.
