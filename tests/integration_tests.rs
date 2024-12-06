use concurrent_server::WebServer;
use reqwest::blocking::get;
use std::thread;
use std::time::Duration;

#[test]
fn test_file_serving() {
    thread::spawn(|| {
        let server = WebServer::new("127.0.0.1:8080", "./static");
        server.run();
    });

    thread::sleep(Duration::from_millis(500));

    let response = get("http://127.0.0.1:8080/index.html").unwrap();
    assert_eq!(response.status(), 200);
    assert!(response.text().unwrap().contains("Welcome to the Rust Static File Server!"));
}

#[test]
fn test_file_not_found() {
    thread::spawn(|| {
        let server = WebServer::new("127.0.0.1:8080", "./static");
        server.run();
    });

    thread::sleep(Duration::from_millis(500));

    let response = get("http://127.0.0.1:8080/missing.html").unwrap();
    assert_eq!(response.status(), 404);
}
