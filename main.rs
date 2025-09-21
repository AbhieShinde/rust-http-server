use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Handles a single client connection.
///
/// This function reads the incoming request from the TCP stream,
/// performs minimal parsing to check if it's a GET request to the root path ("/"),
/// and writes a hardcoded "Hello, World!" HTTP response back to the stream.
///
/// # Arguments
/// * `stream` - A mutable TcpStream representing the connection to the client.
fn handle_connection(mut stream: TcpStream) {
    // Create a buffer to store the incoming request data.
    // 1024 bytes is enough for a simple GET request's headers.
    let mut buffer = [0; 1024];

    // Read data from the stream into the buffer.
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Convert the byte buffer into a string for inspection.
            // We use from_utf8_lossy to prevent panics on invalid UTF-8.
            let request_str = String::from_utf8_lossy(&buffer[..]);

            // Very basic request parsing. We only care about GET requests to the root path.
            if request_str.starts_with("GET / HTTP/1.1") || request_str.starts_with("GET / HTTP/1.0") {
                // If it's the correct request, prepare a valid HTTP 200 OK response.
                let body = "Hello, World!";
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                    body.len(),
                    body
                );

                // Write the response back to the client.
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed to write response: {}", e);
                }
            } else {
                // For any other request, send a 404 Not Found response.
                let body = "Not Found";
                let response = format!(
                    "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                    body.len(),
                    body
                );
                 if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed to write 404 response: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }

    // Ensure all buffered data is sent.
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush stream: {}", e);
    }
}

/// Main function to set up and run the TCP server.
fn main() {
    // Bind the TCP listener to address 0.0.0.0 on port 8080.
    // This allows connections from any network interface.
    let listener = match TcpListener::bind("0.0.0.0:8080") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 8080: {}", e);
            return;
        }
    };

    println!("Server listening on port 8080...");

    // Iterate over incoming connections, accepting them one by one.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // For each valid connection, spawn a new thread to handle it.
                // This allows the server to handle multiple connections concurrently.
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
