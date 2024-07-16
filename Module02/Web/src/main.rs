use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use thiserror::Error;
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::WebSocketStream;
use futures_util::{StreamExt, SinkExt};
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] WsError),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Message processing error: {0}")]
    MessageProcessingError(String),
    
    #[error("Unexpected disconnection")]
    UnexpectedDisconnection,
}

type Peers = Arc<RwLock<HashMap<String, Arc<Mutex<WebSocketStream<TcpStream>>>>>>;

async fn handle_connection(username: String, websocket: WebSocketStream<TcpStream>, peers: Peers) -> Result<(), MyError> {
    let mut write_guard = peers.write().await;
    write_guard.insert(username.clone(), Arc::new(Mutex::new(websocket)));
    println!("User {} connected.", username);
    Ok(())
}

async fn process_messages(peers: Peers) -> Result<(), MyError> {
    let read_guard = peers.read().await;
    for (username, websocket) in read_guard.iter() {
        let websocket = Arc::clone(websocket);
        let username = username.clone();
        tokio::spawn(async move {
            if let Err(e) = process_single_message(&username, websocket).await {
                eprintln!("Error processing message for {}: {:?}", username, e);
            }
        });
    }
    Ok(())
}

async fn process_single_message(username: &String, websocket: Arc<Mutex<WebSocketStream<TcpStream>>>) -> Result<(), MyError> {
    let mut websocket = websocket.lock().await;
    while let Some(message) = websocket.next().await {
        match message {
            Ok(msg) => {
                println!("Received a message from {}: {:?}", username, msg);
                websocket.send(msg).await?;
            }
            Err(e) => {
                eprintln!("Error receiving message from {}: {:?}", username, e);
                return Err(MyError::MessageProcessingError(format!("Error receiving message: {:?}", e)));
            }
        }
    }
    Ok(())
}

async fn handle_disconnection(username: String, peers: Peers) -> Result<(), MyError> {
    let mut write_guard = peers.write().await;
    write_guard.remove(&username);
    println!("User {} disconnected.", username);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), MyError> {
    let peers: Peers = Arc::new(RwLock::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (stream, _) = listener.accept().await?;
        let peers = Arc::clone(&peers);

        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(websocket) => {
                    let username = Uuid::new_v4().to_string();
                    if let Err(e) = handle_connection(username.clone(), websocket, peers.clone()).await {
                        eprintln!("Error handling connection for {}: {:?}", username, e);
                    }

                    // Start processing messages for the new connection
                    if let Err(e) = process_messages(peers.clone()).await {
                        eprintln!("Error processing messages: {:?}", e);
                    }

                    // Handle disconnection
                    if let Err(e) = handle_disconnection(username, peers.clone()).await {
                        eprintln!("Error handling disconnection: {:?}", e);
                    }
                }
                Err(e) => eprintln!("Error accepting connection: {:?}", e),
            }
        });
    }
}
