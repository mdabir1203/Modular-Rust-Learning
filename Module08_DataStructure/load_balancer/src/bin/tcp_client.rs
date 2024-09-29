use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8003")?;
    println!("Connected to load balancer!");

    // Send a test message to the load balancer
    let message = b"Test message from client";
    stream.write_all(message)?;
    println!("Sent: {}", String::from_utf8_lossy(message));

    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

    Ok(())
}

