use concurrent_server::WebServer;

fn main() {
    let server = WebServer::new("127.0.0.1:5005", "./static");
    server.run();
}
