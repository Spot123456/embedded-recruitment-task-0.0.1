use multithreaded_server::server;
use std::sync::Arc;
use env_logger;
use log::info;

fn main() {
    env_logger::init();

    let server = Arc::new(server::Server::new("localhost:8080").expect("Failed to create server"));
    info!("Server is running on localhost:8080"); // Add this line
    server.run().expect("Server failed");
}