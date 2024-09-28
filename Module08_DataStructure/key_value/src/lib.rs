use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

// Define a module named key_value_store
pub mod key_value_store {
    use super::*;

    // Define an enum to represent different operations on the key-value store
    #[derive(Serialize, Deserialize, Debug)]
    pub enum Operation {
        Put { key: String, value: String }, // Insert or update a key-value pair
        Get { key: String }, // Retrieve the value for a given key
        Delete { key: String }, // Remove a key-value pair
    }

    // Define a struct to represent a request to the key-value store
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Request {
        pub operation: Operation, // The operation to be performed
    }

    // Define a struct to represent a response from the key-value store
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Response {
        pub success: bool, // Indicates if the operation was successful
        pub message: Option<String>, // Optional message, e.g., the value for a Get operation
    }

    // Define a public struct named KeyValueStore
    pub struct KeyValueStore {
        // store is a thread-safe, shared, and mutable collection of hash maps
        store: Arc<Mutex<Vec<HashMap<String, String>>>>,
        // bucket_count stores the number of buckets in the key-value store
        bucket_count: usize,
    }

    impl KeyValueStore {
        // Constructor to create a new KeyValueStore with the specified number of buckets
        pub fn new(bucket_count: usize) -> Self {
            // Initialize the store with empty hash maps
            let mut buckets = Vec::with_capacity(bucket_count);
            for _ in 0..bucket_count {
                buckets.push(HashMap::new());
            }
            KeyValueStore {
                // Wrap the store in Arc and Mutex for thread-safe access
                store: Arc::new(Mutex::new(buckets)),
                bucket_count,
            }
        }

        // Hash function to determine the bucket index for a given key
        fn hash_key(&self, key: &str) -> usize {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            hasher.finish() as usize % self.bucket_count
        }

        // Method to insert or update a key-value pair in the store
        pub fn put(&self, key: String, value: String) -> bool {
            let bucket_index = self.hash_key(&key);
            let mut store = self.store.lock().unwrap();
            store[bucket_index].insert(key, value);
            true
        }

        // Method to retrieve the value associated with a given key
        pub fn get(&self, key: &str) -> Option<String> {
            let bucket_index = self.hash_key(key);
            let store = self.store.lock().unwrap();
            store[bucket_index].get(key).cloned()
        }

        // Method to delete a key-value pair from the store
        pub fn delete(&self, key: &str) -> bool {
            let bucket_index = self.hash_key(key);
            let mut store = self.store.lock().unwrap();
            store[bucket_index].remove(key).is_some()
        }
    }

    // Define a struct to represent the key-value server
    pub struct KeyValueServer {
        kv_store: Arc<KeyValueStore>, // Shared key-value store
    }

    impl KeyValueServer {
        // Constructor to create a new KeyValueServer with the specified number of buckets
        pub fn new(bucket_count: usize) -> Self {
            KeyValueServer {
                kv_store: Arc::new(KeyValueStore::new(bucket_count)),
            }
        }

        // Method to handle client connections
        async fn handle_client(&self, mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
            let mut buffer = [0; 1024];

            loop {
                // Read data from the client
                let n = stream.read(&mut buffer).await?;
                if n == 0 {
                    return Ok(());
                }

                // Deserialize the request from the client
                let request: Request = serde_json::from_slice(&buffer[..n])?;
                // Match the operation and perform the corresponding action
                let response = match request.operation {
                    Operation::Put { key, value } => {
                        let success = self.kv_store.put(key, value);
                        Response { success, message: None }
                    }
                    Operation::Get { key } => {
                        let value = self.kv_store.get(&key);
                        Response { success: value.is_some(), message: value }
                    }
                    Operation::Delete { key } => {
                        let success = self.kv_store.delete(&key);
                        Response { success, message: None }
                    }
                };

                // Serialize the response and send it back to the client
                let response_json = serde_json::to_string(&response)?;
                stream.write_all(response_json.as_bytes()).await?;
            }
        }

        // Method to run the server and accept client connections
        pub async fn run(&self, listener: TcpListener) -> Result<(), Box<dyn std::error::Error>> {
            loop {
                // Accept a new client connection
                let (socket, _) = listener.accept().await?;
                let server_clone = Arc::clone(&self.kv_store);

                // Spawn a new task to handle the client connection
                tokio::spawn(async move {
                    let server = KeyValueServer { kv_store: server_clone };
                    if let Err(e) = server.handle_client(socket).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
        }
    }

    // Unit tests for the KeyValueStore
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_put_and_get() {
            let store = KeyValueStore::new(12);
            assert!(store.put("key1".to_string(), "value1".to_string()));
            assert_eq!(store.get("key1"), Some("value1".to_string()));
        }

        #[test]
        fn test_delete() {
            let store = KeyValueStore::new(12);
            store.put("key2".to_string(), "value2".to_string());
            assert!(store.delete("key2"));
            assert_eq!(store.get("key2"), None);
        }

        #[test]
        fn test_nonexistent_key() {
            let store = KeyValueStore::new(12);
            assert_eq!(store.get("nonexistent"), None);
        }
    }
}