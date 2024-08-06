# Number Guessing Game

This is a simple number guessing game written in Rust. The program generates a random number between 1 and 100, and the user has to guess the number. The program provides feedback on whether the guess is too high, too low, or correct.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

2. Clone this repository:

    ```sh
    git clone https://github.com/yourusername/number-guessing-game.git
    ```

3. Navigate to the project directory:

    ```sh
    cd number-guessing-game
    ```

4. Build the project:

    ```sh
    cargo build
    ```

## Usage

To run the game, use the following command:

```sh
cargo run
```

Follow the on-screen prompts to guess the number and receive feedback.

## Features

- Random number generation: The program generates a random number between 1 and 100.
- User input: Prompts the user to enter a guess.
- Feedback: Provides feedback on whether the guess is too high, too low, or correct.
- Loop: Continues until the user guesses the correct number.

## How It Works

### Random Number Generation

The program uses the `rand` crate to generate a random number between 1 and 100.

### User Input

The program reads user input from the command line using the `std::io` library.

### Feedback

After the user enters a guess, the program compares it to the randomly generated number and provides feedback on whether the guess is too high, too low, or correct.

### Main Loop

The game continues in a loop until the user guesses the correct number. It prompts the user for input, checks the guess, and provides feedback accordingly.

## Conclusion

The Number Guessing Game is a fun and interactive way to practice Rust programming. It demonstrates basic concepts such as random number generation, user input handling, conditional logic, and loops. Feel free to modify and extend the game to add new features or improve the user experience. Happy guessing!

## License

This project is licensed under the terms of the MIT license. See the [MIT](https://opensource.org/licenses/MIT) license for more information.
