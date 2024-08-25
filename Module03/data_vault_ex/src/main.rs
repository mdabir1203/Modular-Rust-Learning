use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Define a result type for our operations
type Result<T> = std::result::Result<T, String>;

pub struct KeyValueStore {
    store: Arc<RwLock<HashMap<String, String>>>,
}

impl KeyValueStore {
    // Initialize a new KeyValueStore
    pub fn new() -> Self {
        KeyValueStore {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Insert a key-value pair
    pub fn insert(&self, key: String, value: String) -> Result<()> {
        match self.store.write() {
            Ok(mut guard) => {
                guard.insert(key, value);
                Ok(())
            },
            Err(e) => Err(format!("Failed to acquire write lock: {}", e)),
        }
    }

    // Retrieve a value given a key
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        match self.store.read() {
            Ok(guard) => Ok(guard.get(key).cloned()),
            Err(e) => Err(format!("Failed to acquire read lock: {}", e)),
        }
    }

    // Remove a key-value pair
    pub fn remove(&self, key: &str) -> Result<Option<String>> {
        match self.store.write() {
            Ok(mut guard) => Ok(guard.remove(key)),
            Err(e) => Err(format!("Failed to acquire write lock: {}", e)),
        }
    }

    // Example usage might include checking if a key exists
    pub fn contains(&self, key: &str) -> Result<bool> {
        self.get(key).map(|v| v.is_some())
    }
}

// adding a main function to test the above key value store 

fn main() {
    // Initialize the key-value store
    let store = KeyValueStore::new();

    // Insert some key-value pairs
    match store.insert("key1".to_string(), "value1".to_string()) {
        Ok(_) => println!("Inserted key1 -> value1"),
        Err(e) => println!("Error inserting key1: {}", e),
    }

    match store.insert("key2".to_string(), "value2".to_string()) {
        Ok(_) => println!("Inserted key2 -> value2"),
        Err(e) => println!("Error inserting key2: {}", e),
    }

    // Retrieve a value
    match store.get("key1") {
        Ok(Some(value)) => println!("Retrieved key1 -> {}", value),
        Ok(None) => println!("key1 not found"),
        Err(e) => println!("Error retrieving key1: {}", e),
    }

    // Check if a key exists
    match store.contains("key2") {
        Ok(true) => println!("key2 exists"),
        Ok(false) => println!("key2 does not exist"),
        Err(e) => println!("Error checking key2: {}", e),
    }

    // Remove a key-value pair
    match store.remove("key1") {
        Ok(Some(value)) => println!("Removed key1 -> {}", value),
        Ok(None) => println!("key1 not found"),
        Err(e) => println!("Error removing key1: {}", e),
    }

    // Try to retrieve the removed key
    match store.get("key1") {
        Ok(Some(value)) => println!("Retrieved key1 -> {}", value),
        Ok(None) => println!("key1 not found"),
        Err(e) => println!("Error retrieving key1: {}", e),
    }
}

// Explanation of design choices:
// - `Arc` (Atomic Reference Counting) with `RwLock` allows for shared ownership with safe concurrent access.
// - Using `String` for both key and value for simplicity, but this could be generic in a real-world scenario.
// - Error handling with `Result` allows for clear propagation of errors to the caller.