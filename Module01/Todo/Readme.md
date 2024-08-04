# Simple Todo CLI Application

This Rust application is a straightforward command-line interface (CLI) for managing a todo list. It allows users to add todo items, display all todo items, mark items as completed, and exit the program. This README provides an overview of the application's functionality, including a breakdown of its syntax and core principles.

## Features

Add todo items with a title.
Display all todo items with their status (completed or not).
Mark todo items as completed.
Exit the application.

## How It Works

### Main Components

Structs: Defines the structure of a todo item.
Functions: Implements the logic for adding, displaying, and marking todo items as completed.
Loop: Keeps the application running until the user decides to exit.
Syntax Breakdown
Struct Definition

### struct TodoItem: Defines a new structure named TodoItem with three fields: id, title, and completed

`id:` A unique identifier for each todo item (u32).
`title:` The title or description of the todo item (String).
`completed:` A boolean flag indicating whether the item is completed (bool).
Function to Add Todo Items
`fn add_todo_item:` A function that takes an id and title as arguments and returns a new TodoItem.
`TodoItem { id, title, completed: false }:` Creates a new TodoItem instance with the specified id and title, and sets completed to false.

### Function to Display Todo Items

Iterates over each TodoItem in the provided vector and prints its details.

### Function to Mark Todo Items as Completed

Iterates over the vector of TodoItems and marks the item with the specified id as completed.

## Reading User Input

Prompts the user for input, flushes the standard output to ensure the prompt is displayed, then reads a line of input from the user.

### Main Loop

The main loop of the application displays a menu of options, reads the user's choice, and performs the corresponding action.
Running the Application
To run this application, you need Rust and Cargo installed on your machine. Clone the repository, navigate to the project directory in your terminal, and run cargo run.

## Conclusion

This simple Rust CLI application demonstrates basic Rust syntax and principles, including struct definitions, loops, user input handling, and vector manipulation. It serves as a practical introduction to building CLI applications with Rust.
