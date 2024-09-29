use tokio::net::{TcpListener, TcpStream}; // Importing necessary modules from tokio for networking
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Importing asynchronous read and write extensions
use std::collections::HashMap; // Importing HashMap for key-value storage
use std::sync::{Arc, Mutex}; // Importing Arc and Mutex for thread-safe shared state
use std::hash::{Hash, Hasher}; // Importing traits for hashing
use std::collections::hash_map::DefaultHasher; // Importing default hasher
use serde::{Serialize, Deserialize}; // Importing serde for serialization and deserialization

// Define an enum to represent different operations on the key-value store
#[derive(Serialize, Deserialize, Debug)]
enum Operation {
    Put { key: String, value: String }, // Insert or update a key-value pair
    Get { key: String }, // Retrieve the value for a given key
    Delete { key: String }, // Remove a key-value pair
}

// Define a struct to represent a request to the key-value store
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    operation: Operation, // The operation to be performed
}

// Define a struct to represent a response from the key-value store
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    success: bool, // Indicates if the operation was successful
    message: Option<String>, // Optional message, e.g., the value for a Get operation
}

// Define a struct named KeyValueStore
struct KeyValueStore {
    store: Arc<Mutex<Vec<HashMap<String, String>>>>, // Thread-safe, shared, and mutable collection of hash maps
    bucket_count: usize, // Number of buckets in the key-value store
}

impl KeyValueStore {
    // Constructor to create a new KeyValueStore with the specified number of buckets
    fn new(bucket_count: usize) -> Self {
        let mut buckets = Vec::with_capacity(bucket_count); // Initialize the store with empty hash maps
        for _ in 0..bucket_count {
            buckets.push(HashMap::new());
        }
        KeyValueStore {
            store: Arc::new(Mutex::new(buckets)), // Wrap the store in Arc and Mutex for thread-safe access
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
    fn put(&self, key: String, value: String) -> bool {
        let bucket_index = self.hash_key(&key);
        let mut store = self.store.lock().unwrap();
        store[bucket_index].insert(key, value);
        true
    }

    // Method to retrieve the value associated with a given key
    fn get(&self, key: &str) -> Option<String> {
        let bucket_index = self.hash_key(key);
        let store = self.store.lock().unwrap();
        store[bucket_index].get(key).cloned()
    }

    // Method to delete a key-value pair from the store
    fn delete(&self, key: &str) -> bool {
        let bucket_index = self.hash_key(key);
        let mut store = self.store.lock().unwrap();
        store[bucket_index].remove(key).is_some()
    }
}

// Define a struct to represent the key-value server
struct KeyValueServer {
    kv_store: Arc<KeyValueStore>, // Shared key-value store
}

impl KeyValueServer {
    // Constructor to create a new KeyValueServer with the specified number of buckets
    fn new(bucket_count: usize) -> Self {
        KeyValueServer {
            kv_store: Arc::new(KeyValueStore::new(bucket_count)),
        }
    }

    // Method to handle client connections
    async fn handle_client(&self, mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = [0; 1024]; // Buffer to store data from the client

        loop {
            let n = stream.read(&mut buffer).await?; // Read data from the client
            if n == 0 {
                return Ok(()); // If no data, return
            }

            let request: Request = serde_json::from_slice(&buffer[..n])?; // Deserialize the request from the client
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

            let response_json = serde_json::to_string(&response)?; // Serialize the response
            stream.write_all(response_json.as_bytes()).await?; // Send the response back to the client
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Arc::new(KeyValueServer::new(12)); // Create a new key-value server with 12 buckets
    let listener = TcpListener::bind("127.0.0.1:8081").await?; // Bind the server to the specified address

    println!("Server listening on 127.0.0.1:8081");

    loop {
        let (socket, _) = listener.accept().await?; // Accept a new client connection
        let server_clone = Arc::clone(&server); // Clone the server for the new client

        tokio::spawn(async move {
            if let Err(e) = server_clone.handle_client(socket).await {
                eprintln!("Error handling client: {}", e); // Handle any errors
            }
        });
    }
}