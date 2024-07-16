use std::net::TcpListener;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server is running on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let mut stream = stream?;
        handle_connection(&mut stream)?;
    }

    Ok(())
}

fn handle_connection(stream: &mut std::net::TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, contents) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", include_str!("test.html"))
    } else {
        ("HTTP/1.1 404 NOT FOUND", include_str!("404.html"))
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
