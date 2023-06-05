use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    stream.write_all(b"hostname").unwrap();
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection closed by client
                println!("Client disconnected");
                break;
            }
            Ok(bytes_read) => {
                println!("Client connected");
                let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received request: {}", request);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
                stream.write_all(input.trim().as_bytes()).expect("Failed to write to stream");
            }
            Err(err) => {
                eprintln!("Error reading from stream: {}", err);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
