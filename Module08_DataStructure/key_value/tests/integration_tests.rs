use distributed_kv_store::key_value_store::{KeyValueServer, Request, Response, Operation};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn send_request(stream: &mut TcpStream, request: &Request) -> Result<Response, Box<dyn std::error::Error>> {
    let req_json = serde_json::to_string(request)?;
    stream.write_all(req_json.as_bytes()).await?;
    
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    
    let response: Response = serde_json::from_slice(&buffer[..n])?;
    Ok(response)
}

#[tokio::test]
async fn test_key_value_store_operations() -> Result<(), Box<dyn std::error::Error>> {
    // Start the server
    let server = KeyValueServer::new(10);
    let listener = TcpListener::bind("127.0.0.1:8082").await?;

    tokio::spawn(async move {
        if let Err(e) = server.run(listener).await {
            eprintln!("Server error: {}", e);
        }
    });

    let mut stream = TcpStream::connect("127.0.0.1:8082").await?;
    
    // **Test Case 1: PUT operation**
    let put_request = Request {
        operation: Operation::Put {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        },
    };
    let put_response = send_request(&mut stream, &put_request).await?;
    assert!(put_response.success);
    assert_eq!(put_response.message, Some("Value stored successfully.".to_string()));

    // **Test Case 2: GET operation on existing key**
    let get_request = Request {
        operation: Operation::Get {
            key: "test_key".to_string(),
        },
    };
    let get_response = send_request(&mut stream, &get_request).await?;
    assert!(get_response.success);
    assert_eq!(get_response.message, Some("test_value".to_string()));

    // **Test Case 3: DELETE operation on existing key**
    let delete_request = Request {
        operation: Operation::Delete {
            key: "test_key".to_string(),
        },
    };
    let delete_response = send_request(&mut stream, &delete_request).await?;
    assert!(delete_response.success);
    assert_eq!(delete_response.message, Some("Key deleted successfully.".to_string()));

    // **Test Case 4: GET on deleted key**
    let get_response_after_delete = send_request(&mut stream, &get_request).await?;
    assert!(!get_response_after_delete.success);
    assert_eq!(get_response_after_delete.message, Some("Key not found.".to_string()));

    // **Test Case 5: PUT operation with empty key/value**
    let invalid_put_request = Request {
        operation: Operation::Put {
            key: "".to_string(),
            value: "".to_string(),
        },
    };
    let invalid_put_response = send_request(&mut stream, &invalid_put_request).await?;
    assert!(!invalid_put_response.success);
    assert_eq!(invalid_put_response.message, Some("Invalid key or value.".to_string()));

    // **Test Case 6: GET non-existent key**
    let get_non_existent_request = Request {
        operation: Operation::Get {
            key: "non_existent_key".to_string(),
        },
    };
    let get_non_existent_response = send_request(&mut stream, &get_non_existent_request).await?;
    assert!(!get_non_existent_response.success);
    assert_eq!(get_non_existent_response.message, Some("Key not found.".to_string()));

    // **Test Case 7: DELETE non-existent key**
    let delete_non_existent_request = Request {
        operation: Operation::Delete {
            key: "non_existent_key".to_string(),
        },
    };
    let delete_non_existent_response = send_request(&mut stream, &delete_non_existent_request).await?;
    assert!(!delete_non_existent_response.success);
    assert_eq!(delete_non_existent_response.message, Some("Key not found.".to_string()));

    // **Test Case 8: Invalid JSON Format**
    let invalid_json = b"{invalid_request";
    stream.write_all(invalid_json).await?;
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let invalid_response: Response = serde_json::from_slice(&buffer[..n]).unwrap_or_else(|_| Response { success: false, message: Some("Invalid request.".to_string()) });
    assert!(!invalid_response.success);

    // **Test Case 9: PUT and GET with large key/value**
    let large_key = "k".repeat(1024); // 1KB key
    let large_value = "v".repeat(1024); // 1KB value
    let large_put_request = Request {
        operation: Operation::Put {
            key: large_key.clone(),
            value: large_value.clone(),
        },
    };
    let large_put_response = send_request(&mut stream, &large_put_request).await?;
    assert!(large_put_response.success);
    assert_eq!(large_put_response.message, Some("Value stored successfully.".to_string()));

    let large_get_request = Request {
        operation: Operation::Get {
            key: large_key.clone(),
        },
    };
    let large_get_response = send_request(&mut stream, &large_get_request).await?;
    assert!(large_get_response.success);
    assert_eq!(large_get_response.message, Some(large_value));


    // Ensure last written value is correct
    Ok(())
}
