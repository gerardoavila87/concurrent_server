use concurrent_server::WebServer;

fn main() {
    let server = WebServer::new("127.0.0.1:8080", "./static");
    server.run();
}
