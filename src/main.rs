use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5555")?;

    while let Some(Ok(mut stream)) = listener.incoming().next() {
        stream.write(b"Welcome to my echo service. Please write a message:\n")?;
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        let request = std::str::from_utf8(&buffer).unwrap();

        println!("Got Answer: {request}");

        stream.write(format!("Echo: {}", request).as_bytes())?;
    }

    Ok(())
}
