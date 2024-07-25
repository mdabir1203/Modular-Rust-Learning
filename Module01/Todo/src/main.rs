use std::io::{self, Write};
use std::num::ParseIntError;

#[derive(Debug)]
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}

fn add_todo_item(id: u32, title: String) -> TodoItem {
    TodoItem {
        id,
        title,
        completed: false,
    }
}

fn display_todo_items(items: &[TodoItem]) {
    for item in items {
        println!(
            "{}: {} [{}]",
            item.id,
            item.title,
            if item.completed { "x" } else { " " }
        );
    }
}

fn mark_as_completed(items: &mut [TodoItem], id: u32) -> Result<(), String> {
    items
        .iter_mut()
        .find(|item| item.id == id)
        .map(|item| {
            item.completed = true;
            Ok(())
        })
        .unwrap_or_else(|| Err(format!("Item with ID {} not found", id)))
}

fn read_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn parse_u32(input: &str) -> Result<u32, ParseIntError> {
    input.parse::<u32>()
}

fn main() -> io::Result<()> {
    let mut todo_items = Vec::new();

    loop {
        println!("\n1. Add Todo Item");
        println!("2. Display Todo Items");
        println!("3. Mark Item as Completed");
        println!("4. Exit");

        let choice = read_input("Enter your choice: ")?;

        match choice.as_str() {
            "1" => {
                let title = read_input("Enter the title: ")?;
                let id = todo_items.len() as u32 + 1;
                todo_items.push(add_todo_item(id, title));
                println!("Todo item added successfully!");
            }
            "2" => {
                if todo_items.is_empty() {
                    println!("No todo items to display.");
                } else {
                    display_todo_items(&todo_items);
                }
            }
            "3" => {
                if todo_items.is_empty() {
                    println!("No todo items to mark as completed.");
                } else {
                    let id_input = read_input("Enter the ID of the item to mark as completed: ")?;
                    match parse_u32(&id_input) {
                        Ok(id) => match mark_as_completed(&mut todo_items, id) {
                            Ok(()) => println!("Item marked as completed successfully!"),
                            Err(e) => println!("Error: {}", e),
                        },
                        Err(_) => println!("Invalid ID. Please enter a valid number."),
                    }
                }
            }
            "4" => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please enter a number between 1 and 4."),
        }
    }

    Ok(())
}