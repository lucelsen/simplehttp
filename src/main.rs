use std::io::{Read, Write};
use std::net::TcpListener;
use std::str::FromStr;

#[derive(PartialEq)]
enum HttpMethod {
    GET,
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            _ => Err(()),
        }
    }
}

struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
}

impl TryFrom<&str> for HttpRequest {
    type Error = ();

    fn try_from(request: &str) -> Result<Self, Self::Error> {
        let method;
        let path;

        match request.lines().next() {
            None => return Err(()),
            Some(first_line) => {
                let mut chunks = first_line.split_whitespace();

                match chunks.next() {
                    None => return Err(()),
                    Some(first_word) => method = HttpMethod::from_str(first_word)?,
                }

                match chunks.next() {
                    None => return Err(()),
                    Some(second_word) => path = second_word.to_string(),
                }

                // NOTE: The third word would be the HTTP Version
            }
        };

        // NOTE: More Lines could contain HTTP headers or contents

        Ok(HttpRequest { method, path })
    }
}

const SUCESS_STRING: &str = "HTTP/1.1 200 OK\r\n\r\n";
const FAILURE_STRING: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

fn handle_request(request: &str) -> &str {
    match HttpRequest::try_from(request) {
        Ok(http) => {
            if http.path == "/" && http.method == HttpMethod::GET {
                SUCESS_STRING
            } else {
                FAILURE_STRING
            }
        }
        Err(_) => FAILURE_STRING,
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5555")?;

    while let Some(Ok(mut stream)) = listener.incoming().next() {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        let request = std::str::from_utf8(&buffer).unwrap();
        println!("------ Request\n{request}\n------ End Request");

        let response = handle_request(request);
        println!("------ Response\n{response}\n------ End Response");

        stream.write(response.as_bytes())?;
    }

    Ok(())
}
