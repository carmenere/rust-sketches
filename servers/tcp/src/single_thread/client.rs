use std::net::TcpStream;

// std::net::TcpStream implements Read and Write traits
use std::io::{Read, Write, Error};

pub fn run() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("localhost:5555")?;
    stream.write("Hello, Server\n".as_bytes())?;
    let mut buf = [0; 1024];
    stream.read(&mut buf)?;
    print!("Got response from server: {}", String::from_utf8_lossy(&buf));
    Ok(())
}
