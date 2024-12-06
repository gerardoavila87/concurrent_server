use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use mime_guess::from_path;

pub struct WebServer {
    address: String,
    static_dir: String,
}

impl WebServer {
    pub fn new(address: &str, static_dir: &str) -> Self {
        WebServer {
            address: address.to_string(),
            static_dir: static_dir.to_string(),
        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind address");
        let static_dir = Arc::new(self.static_dir.clone());

        println!("Server running at http://{}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let static_dir = Arc::clone(&static_dir);
                    thread::spawn(move || {
                        handle_connection(stream, &static_dir);
                    });
                }
                Err(e) => eprintln!("Failed to accept connection: {}", e),
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, static_dir: &str) {
    let mut buffer = [0; 1024];
    if stream.read(&mut buffer).is_err() {
        eprintln!("Failed to read from connection");
        return;
    }

    let request = String::from_utf8_lossy(&buffer);
    let request_line = request.lines().next().unwrap_or("");
    let mut parts = request_line.split_whitespace();

    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    if method != "GET" {
        send_response(&mut stream, "405 Method Not Allowed", None);
        return;
    }

    let mut file_path = PathBuf::from(static_dir);
    if path == "/" {
        file_path.push("index.html");
    } else {
        file_path.push(&path[1..]);
    }

    if file_path.exists() && file_path.is_file() {
        let mime_type = from_path(&file_path).first_or_text_plain();
        let mut file = match fs::File::open(&file_path) {
            Ok(file) => file,
            Err(_) => {
                send_response(&mut stream, "500 Internal Server Error", None);
                return;
            }
        };

        let mut content = Vec::new();
        if file.read_to_end(&mut content).is_ok() {
            send_response(&mut stream, "200 OK", Some((&mime_type.to_string(), &content)));
        } else {
            send_response(&mut stream, "500 Internal Server Error", None);
        }
    } else {
        send_response(&mut stream, "404 Not Found", Some(("text/html", b"File not found")));
    }
}

fn send_response(stream: &mut TcpStream, status: &str, body: Option<(&str, &[u8])>) {
    let mut response = format!("HTTP/1.1 {}\r\n", status);

    if let Some((content_type, body)) = body {
        response.push_str(&format!("Content-Type: {}\r\n", content_type));
        response.push_str(&format!("Content-Length: {}\r\n", body.len()));
        response.push_str("\r\n");

        // Escribir encabezados primero
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to send headers: {}", e);
            return;
        }

        // Escribir el cuerpo después
        if let Err(e) = stream.write_all(body) {
            eprintln!("Failed to send body: {}", e);
        }
    } else {
        response.push_str("Content-Length: 0\r\n\r\n");

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to send response: {}", e);
        }
    }
}
