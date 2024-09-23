use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::io::{self, Read, Write};
use lazy_static::lazy_static;
use log::{info, error, LevelFilter};
use simplelog::{CombinedLogger, TermLogger, TerminalMode, Config, ColorChoice};
use prometheus::{Counter, Encoder, TextEncoder, Registry};


lazy_static! {
    static ref BACKENDS: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![
        "127.0.0.1:8080",
        "127.0.0.1:8082"
    ]));
}

/// Handles the client connection by establishing a connection to the backend.
fn handle_connection(client_stream: Arc<Mutex<TcpStream>>) {
    let backend_address = {
        let mut backend_pool = BACKENDS.lock().unwrap();
        if backend_pool.is_empty() {
            error!("No backends available.");
            return;
        }
        let selected_backend = backend_pool.remove(0);
        backend_pool.push(selected_backend);
        selected_backend
    };

    let backend_stream = match TcpStream::connect(backend_address) {
        Ok(stream) => Arc::new(Mutex::new(stream)),
        Err(e) => {
            error!("Failed to connect to backend at {}: {}", backend_address, e);
            return;
        }
    };

    let (client_sender, client_receiver) = mpsc::channel();

    // Thread to handle communication from client to backend
    let client_to_backend = {
        let client_stream = Arc::clone(&client_stream);
        thread::spawn(move || {
            let mut buffer = [0; 1024]; // Configurable buffer size
            loop {
                let mut stream = client_stream.lock().unwrap();
                match stream.read(&mut buffer) {
                    Ok(0) => break, // Connection closed
                    Ok(bytes_read) => {
                        if client_sender.send(buffer[..bytes_read].to_vec()).is_err() {
                            break; // Sender dropped
                        }
                    },
                    Err(e) => {
                        error!("Error reading from client: {}", e);
                        break;
                    }
                }
            }
        })
    };

    // Thread to handle communication from backend to client
    let backend_to_client = {
        let backend_stream = Arc::clone(&backend_stream);
        thread::spawn(move || {
            while let Ok(data) = client_receiver.recv() {
                let mut backend_stream = backend_stream.lock().unwrap();
                if backend_stream.write_all(&data).is_err() {
                    error!("Error writing to backend.");
                    break;
                }
            }
        })
    };

    let _ = client_to_backend.join();
    let _ = backend_to_client.join();
}


fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
    ]).unwrap();

    let listener = TcpListener::bind("127.0.0.1:8001").expect("Failed to bind to address");
    info!("Listening on 127.0.0.1:8001");

    for incoming_stream in listener.incoming() {
        match incoming_stream {
            Ok(stream) => {
                let client_stream = Arc::new(Mutex::new(stream));
                thread::spawn(move || {
                    handle_connection(client_stream);
                });
            },
            Err(e) => error!("Error accepting connection: {}", e),
        }
    }
}
