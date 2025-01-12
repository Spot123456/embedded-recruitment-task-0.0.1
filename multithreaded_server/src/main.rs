
mod server;

use std::sync::Arc;
use log::info;
use env_logger;

fn main() {
    env_logger::init();

    let server = Arc::new(server::Server::new("localhost:8080").expect("Failed to create server"));
    server.run().expect("Server failed");
}
