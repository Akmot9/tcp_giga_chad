use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("server:8080")?;

    stream.write_all(b"Hello from client")?;
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    println!("Received: {}", str::from_utf8(&buffer).unwrap().trim_end());
    Ok(())
}
