use std::io::{self, Write};
use std::num::ParseIntError;

#[derive(Debug)]
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}

impl TodoItem {
    fn new(id: u32, title: String) -> Self {
        TodoItem {
            id,
            title,
            completed: false,
        }
    }

    fn mark_as_completed(&mut self) {
        self.completed = true;
    }
}

fn display_todo_items(items: &[TodoItem]) {
    for item in items {
        let completed_status = if item.completed { "x" } else { " " };
        println!("{}: {} [{}]", item.id, item.title, completed_status);
    }
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

/// Handles the "Add Todo Item" functionality.
fn add_todo_item(todo_items: &mut Vec<TodoItem>) -> io::Result<()> {
    let title = read_input("Enter the title: ")?;
    let id = todo_items.len() as u32 + 1;
    todo_items.push(TodoItem::new(id, title));
    println!("Todo item added successfully!");
    Ok(())
}

/// Handles the "Display Todo Items" functionality.
fn display_todo_items_handler(todo_items: &Vec<TodoItem>) {
    if todo_items.is_empty() {
        println!("No todo items to display.");
    } else {
        display_todo_items(todo_items);
    }
}

/// Handles the "Mark Item as Completed" functionality.
fn mark_item_as_completed(todo_items: &mut Vec<TodoItem>) -> io::Result<()> {
    if todo_items.is_empty() {
        println!("No todo items to mark as completed.");
        return Ok(());
    }

    let id_input = read_input("Enter the ID of the item to mark as completed: ")?;
    match parse_u32(&id_input) {
        Ok(id) => {
            if let Some(item) = todo_items.iter_mut().find(|item| item.id == id) {
                item.mark_as_completed();
                println!("Item marked as completed successfully!");
            } else {
                println!("Item with ID {} not found", id);
            }
        }
        Err(_) => println!("Invalid ID. Please enter a valid number."),
    }
    Ok(())
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
            "1" => add_todo_item(&mut todo_items)?,
            "2" => display_todo_items_handler(&todo_items),
            "3" => mark_item_as_completed(&mut todo_items)?,
            "4" => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please enter a number between 1 and 4."),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_item_new() {
        let item = TodoItem::new(1, "Buy groceries".to_string());
        assert_eq!(item.id, 1);
        assert_eq!(item.title, "Buy groceries");
        assert_eq!(item.completed, false);
    }

    #[test]
    fn test_todo_item_mark_as_completed() {
        let mut item = TodoItem::new(1, "Buy groceries".to_string());
        item.mark_as_completed();
        assert_eq!(item.completed, true);
    }

    #[test]
    fn test_display_todo_items_empty() {
        let items: Vec<TodoItem> = Vec::new();
        display_todo_items(&items);
    }

    #[test]
    fn test_display_todo_items() {
        let items = vec![
            TodoItem::new(1, "Buy groceries".to_string()),
            TodoItem::new(2, "Walk the dog".to_string()),
        ];
        display_todo_items(&items);
    }

    #[test]
    fn test_parse_u32() {
        assert_eq!(parse_u32("42").unwrap(), 42);
        assert!(parse_u32("abc").is_err());
    }
}
