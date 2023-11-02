use std::io::{Read, Write};
use std::net::TcpListener;

fn handle_request(_request: &str) -> &str {
    "HTTP/1.1 200 OK\r\n\r\n"
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5555")?;

    while let Some(Ok(mut stream)) = listener.incoming().next() {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        // TODO: Here the request should be parsed in the future
        let request = std::str::from_utf8(&buffer).unwrap();
        println!("------ Request\n{request}\n------ End Request");

        let response = handle_request(request);
        println!("------ Response\n{response}\n------ End Response");

        stream.write(response.as_bytes())?;
    }

    Ok(())
}
