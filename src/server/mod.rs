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
    // Implementación de handle_connection
}
