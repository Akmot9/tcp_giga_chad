use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                // Connexion fermée
                return;
            }
            println!("Received: {}", str::from_utf8(&buffer[..size]).unwrap());
            true
        }
        Err(e) => {
            eprintln!("An error occurred, terminating connection with {}: {}", stream.peer_addr().unwrap(), e);
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}