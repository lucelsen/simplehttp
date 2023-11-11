use std::fmt;
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

const OK_STRING: &str = "200 OK";
const NOTFOUND_STRING: &str = "404 Not Found";

enum HttpResponseStatus {
    Ok,
    NotFound,
}

impl fmt::Display for HttpResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Ok => write!(f, "{}", OK_STRING),
            Self::NotFound => write!(f, "{}", NOTFOUND_STRING),
        }
    }
}

struct HttpResponse {
    pub status: HttpResponseStatus,
}

impl Into<String> for HttpResponse {
    fn into(self) -> String {
        format!("HTTP/1.1 {}\r\n\r\n", self.status.to_string())
    }
}

impl HttpResponse {
    fn new(status: HttpResponseStatus) -> Self {
        HttpResponse { status }
    }
}

fn handle_request(request: &str) -> String {
    let response: HttpResponse = match HttpRequest::try_from(request) {
        Ok(http) => {
            if http.path == "/" && http.method == HttpMethod::GET {
                HttpResponse::new(HttpResponseStatus::Ok)
            } else {
                HttpResponse::new(HttpResponseStatus::NotFound)
            }
        }
        Err(_) => HttpResponse::new(HttpResponseStatus::NotFound),
    };

    response.into()
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
