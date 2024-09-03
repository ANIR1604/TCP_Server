use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream, client_id: usize) {
    let mut buffer = [0; 512];
    println!("Handling client {}", client_id);
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Client {} disconnected", client_id);
                    break;
                }
                println!(
                    "Received from client {}: {}",
                    client_id,
                    String::from_utf8_lossy(&buffer[..bytes_read])
                );
                stream.write(&buffer[..bytes_read]).unwrap();
            }
            Err(e) => {
                println!("Failed to read from client {}: {}", client_id, e);
                break;
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:7878");
    let mut client_id = 0; // Initialize client_id
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                client_id += 1; // Increment client_id for each new connection
                thread::spawn(move || {
                    handle_client(stream, client_id);
                });
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
}