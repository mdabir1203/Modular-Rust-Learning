use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::io::{self, Read, Write};
use lazy_static::lazy_static;
use log::{info, error, warn, LevelFilter};
use simplelog::{CombinedLogger, TermLogger, TerminalMode, Config, ColorChoice};
use std::time::{Duration, Instant};

lazy_static! {
    static ref BACKENDS: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![
        "127.0.0.1:8001",
        "127.0.0.1:8002",
        "127.0.0.1:8003",
        "127.0.0.1:8004"
    ]));
}

/// Check if a backend is available by attempting to establish a connection
/// with a timeout. If the backend is reachable, return `true`. Otherwise, return `false`.
fn check_backend(backend_address: &str) -> bool {
    let timeout_duration = Duration::from_secs(30); // Set a 2-second timeout

    match backend_address.parse::<SocketAddr>() {
        Ok(socket_addr) => {
            let start_time = Instant::now();
            match TcpStream::connect_timeout(&socket_addr, timeout_duration) {
                Ok(_) => {
                    let duration = start_time.elapsed();
                    info!("Backend {} is reachable (Connected in {:?})", backend_address, duration);
                    true
                }
                Err(_) => {
                    warn!("Backend {} is unreachable (Timeout after {:?})", backend_address, timeout_duration);
                    false
                }
            }
        }
        Err(_) => {
            error!("Invalid backend address: {}", backend_address);
            false
        }
    }
}

/// Handles the client connection by establishing a connection to the backend.
fn handle_connection(client_stream: Arc<Mutex<TcpStream>>) {
    // Select the next available backend
    let backend_address = {
        let mut backend_pool = BACKENDS.lock().unwrap();
        if backend_pool.is_empty() {
            error!("No backends available.");
            return;
        }

        let mut selected_backend = backend_pool.remove(0);
        
        // Check if the backend is reachable, otherwise rotate through the pool
        while !check_backend(selected_backend) {
            warn!("Backend {} is down, checking the next one...", selected_backend);
            backend_pool.push(selected_backend); // Rotate the failed backend to the end
            selected_backend = backend_pool.remove(0);
        }

        backend_pool.push(selected_backend); // Rotate the backend
        selected_backend
    };

    // Try to establish a connection to the backend
    let backend_stream = match TcpStream::connect(backend_address) {
        Ok(stream) => Arc::new(Mutex::new(stream)),
        Err(e) => {
            error!("Failed to connect to backend {}: {}", backend_address, e);
            return;
        }
    };

    let (client_sender, client_receiver) = mpsc::channel::<Vec<u8>>();

    // Thread to handle communication from client to backend
    let client_to_backend = {
        let client_stream = Arc::clone(&client_stream);
        let backend_stream = Arc::clone(&backend_stream);

        thread::spawn(move || {
            let mut buffer = [0; 2048]; // Configurable buffer size
            loop {
                let mut client_stream_locked = client_stream.lock().unwrap();
                let mut backend_stream_locked = backend_stream.lock().unwrap();

                match client_stream_locked.read(&mut buffer) {
                    Ok(0) => {
                        // Client closed the connection
                        warn!("Client disconnected");
                        break;
                    }
                    Ok(bytes_read) => {
                        // Forward data to backend
                        if backend_stream_locked.write_all(&buffer[..bytes_read]).is_err() {
                            error!("Error writing to backend. Closing connection.");
                            break;
                        }
                    }
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
        let client_stream = Arc::clone(&client_stream);
        let backend_stream = Arc::clone(&backend_stream);

        thread::spawn(move || {
            let mut buffer = [0; 2048]; // Configurable buffer size
            loop {
                let mut backend_stream_locked = backend_stream.lock().unwrap();
                let mut client_stream_locked = client_stream.lock().unwrap();

                match backend_stream_locked.read(&mut buffer) {
                    Ok(0) => {
                        // Backend closed the connection
                        warn!("Backend {} disconnected", backend_stream_locked.peer_addr().unwrap());
                        break;
                    }
                    Ok(bytes_read) => {
                        // Forward data to client
                        if client_stream_locked.write_all(&buffer[..bytes_read]).is_err() {
                            error!("Error writing to client. Closing connection.");
                            break;
                        }
                    }
                    Err(e) => {
                        error!("Error reading from backend: {}", e);
                        break;
                    }
                }
            }
        })
    };

    let _ = client_to_backend.join();
    let _ = backend_to_client.join();
}

fn main() {
    // Initialize logging
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
    ]).unwrap();

    let listener = TcpListener::bind("127.0.0.1:7500").expect("Failed to bind to address");
    info!("Load balancer listening on 127.0.0.1:7500");

    for incoming_stream in listener.incoming() {
        match incoming_stream {
            Ok(stream) => {
                let client_stream = Arc::new(Mutex::new(stream));
                thread::spawn(move || {
                    handle_connection(client_stream);
                });
            }
            Err(e) => error!("Error accepting connection: {}", e),
        }
    }
}
