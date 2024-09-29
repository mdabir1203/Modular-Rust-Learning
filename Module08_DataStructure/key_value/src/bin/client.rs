use distributed_kv_store::key_value_store::{Request, Response, Operation}; // Importing necessary modules for request, response, and operations
use tokio::net::TcpStream; // Importing TcpStream for network communication
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Importing asynchronous read and write extensions

// Function to parse user input into a Request object
fn parse_input(input: &str) -> Result<Request, String> {
    // Split the input string into parts based on spaces
    let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();
    
    // Match the first part of the input to determine the operation
    match parts[0].to_uppercase().as_str() {
        "PUT" => {
            // PUT operation requires exactly 3 parts: command, key, and value
            if parts.len() != 3 {
                return Err("PUT operation requires a key and a value. Usage: PUT <key> <value>".to_string());
            }
            // Create a Request object for the PUT operation
            Ok(Request {
                operation: Operation::Put {
                    key: parts[1].to_string(),
                    value: parts[2].to_string(),
                },
            })
        },
        "GET" => {
            // GET operation requires exactly 2 parts: command and key
            if parts.len() != 2 {
                return Err("GET operation requires a key. Usage: GET <key>".to_string());
            }
            // Create a Request object for the GET operation
            Ok(Request {
                operation: Operation::Get {
                    key: parts[1].to_string(),
                },
            })
        },
        "DELETE" => {
            // DELETE operation requires exactly 2 parts: command and key
            if parts.len() != 2 {
                return Err("DELETE operation requires a key. Usage: DELETE <key>".to_string());
            }
            // Create a Request object for the DELETE operation
            Ok(Request {
                operation: Operation::Delete {
                    key: parts[1].to_string(),
                },
            })
        },
        // If the command is not recognized, return an error
        _ => Err("Invalid operation. Use PUT, GET, or DELETE.".to_string()),
    }
}

// Main function to run the client
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the key-value store server
    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;
    
    println!("Connected to key-value store server. Type 'help' for usage information.");
    
    loop {
        // Prompt the user for input
        println!("\nEnter command (or 'help' for usage, 'quit' to exit):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        
        // If the user types 'quit', exit the loop
        if input.eq_ignore_ascii_case("quit") {
            println!("Exiting client. Goodbye!");
            break;
        }
        
        // If the user types 'help', display usage information
        if input.eq_ignore_ascii_case("help") {
            println!("Usage:");
            println!("  PUT <key> <value>  - Store a key-value pair");
            println!("  GET <key>          - Retrieve the value for a key");
            println!("  DELETE <key>       - Remove a key-value pair");
            println!("  help               - Show this help message");
            println!("  quit               - Exit the client");
            continue;
        }
        
        // Parse the user input into a Request object
        let request = match parse_input(input) {
            Ok(req) => req,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        
        // Serialize the Request object to JSON and send it to the server
        let request_json = serde_json::to_string(&request)?;
        stream.write_all(request_json.as_bytes()).await?;
        
        // Read the response from the server
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        let response: Response = serde_json::from_slice(&buffer[..n])?;
        
        // Handle the response based on the type of operation
        match &request.operation {
            Operation::Put { key, value } => {
                if response.success {
                    println!("Successfully stored '{}' = '{}'", key, value);
                } else {
                    println!("Failed to store '{}' = '{}'", key, value);
                }
            },
            Operation::Get { key } => {
                if response.success {
                    println!("Value for '{}': {}", key, response.message.unwrap_or_else(|| "No value".to_string()));
                } else {
                    println!("Key '{}' not found", key);
                }
            },
            Operation::Delete { key } => {
                if response.success {
                    println!("Successfully deleted key '{}'", key);
                } else {
                    println!("Failed to delete key '{}'. It may not exist.", key);
                }
            },
        }
    }
    
    Ok(())
}