// src/main.rs

mod dynamic;
mod heap;

use dynamic::DynamicArray;
use heap::MinHeap;
use std::io::{self, Write};

fn main() {
    let mut dynamic_array = DynamicArray::new();
    let mut min_heap = MinHeap::new();

    loop {
        println!("\nChoose an option:");
        println!("1. Dynamic Array Operations");
        println!("2. Min Heap Operations");
        println!("3. Exit");

        let choice = get_input("Enter your choice (1-3): ");
        
        match choice.trim().parse::<u32>() {
            Ok(1) => dynamic_array_menu(&mut dynamic_array),
            Ok(2) => min_heap_menu(&mut min_heap),
            Ok(3) => break,
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn dynamic_array_menu(dynamic_array: &mut DynamicArray) {
    loop {
        println!("\nDynamic Array Menu:");
        println!("1. Append Element");
        println!("2. Get Element");
        println!("3. Display Array");
        println!("4. Back to Main Menu");

        let choice = get_input("Enter your choice (1-4): ");
        
        match choice.trim().parse::<u32>() {
            Ok(1) => {
                let value = get_input("Enter value to append: ");
                if let Ok(num) = value.trim().parse::<i32>() {
                    dynamic_array.append(num);
                    println!("Appended {}", num);
                } else {
                    println!("Invalid number!");
                }
            },
            Ok(2) => {
                let index = get_input("Enter index to get element from: ");
                if let Ok(idx) = index.trim().parse::<usize>() {
                    match dynamic_array.get(idx) {
                        Some(val) => println!("Element at index {} is {}", idx, val),
                        None => println!("Index out of bounds!"),
                    }
                } else {
                    println!("Invalid index!");
                }
            },
            Ok(3) => dynamic_array.display(),
            Ok(4) => break,
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn min_heap_menu(min_heap: &mut MinHeap) {
    loop {
        println!("\nMin Heap Menu:");
        println!("1. Insert Element");
        println!("2. Extract Minimum");
        println!("3. Display Heap");
        println!("4. Back to Main Menu");

        let choice = get_input("Enter your choice (1-4): ");
        
        match choice.trim().parse::<u32>() {
            Ok(1) => {
                let value = get_input("Enter value to insert into heap: ");
                if let Ok(num) = value.trim().parse::<i32>() {
                    min_heap.insert(num);
                    println!("Inserted {}", num);
                } else {
                    println!("Invalid number!");
                }
            },
            Ok(2) => match min_heap.extract_min() {
                Some(min_val) => println!("Extracted minimum value: {}", min_val),
                None => println!("Heap is empty!"),
            },
            Ok(3) => min_heap.display(),
            Ok(4) => break,
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure prompt is printed before input
    let mut input = String::new();
    
    io::stdin()
       .read_line(&mut input)
       .expect("Failed to read line");
    
    input
}
