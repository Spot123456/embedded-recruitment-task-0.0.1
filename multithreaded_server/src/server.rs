
use std::{
    net::{TcpListener, TcpStream},
    io::{self, Read, Write},
    sync::{Arc, Mutex},
    thread,
};
use threadpool::ThreadPool;
use log::{info, error};
use prost::Message;
use embedded_recruitment_task::message::{EchoMessage, ServerMessage};

pub struct Server {
    listener: TcpListener,
    is_running: Arc<Mutex<bool>>,
}

impl Server {
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        let is_running = Arc::new(Mutex::new(true));
        Ok(Server { listener, is_running })
    }

    pub fn run(&self) -> io::Result<()> {
        let pool = ThreadPool::new(4); // Adjust the thread pool size as needed
        info!("Server is running on {}", self.listener.local_addr()?);

        for stream in self.listener.incoming() {
            let stream = stream?;
            let is_running = Arc::clone(&self.is_running);

            pool.execute(move || {
                if let Err(e) = handle_client(stream) {
                    error!("Error handling client: {}", e);
                }
            });

            if !*is_running.lock().unwrap() {
                break;
            }
        }
        info!("Server stopped.");
        Ok(())
    }

    pub fn stop(&self) {
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
        info!("Shutdown signal sent.");
    }
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        info!("Client disconnected.");
        return Ok(());
    }

    if let Ok(message) = EchoMessage::decode(&buffer[..bytes_read]) {
        info!("Received: {}", message.content);
        let response = ServerMessage { message: Some(server_message::Message::EchoMessage(message)) };
        let mut payload = Vec::new();
        response.encode(&mut payload).unwrap();
        stream.write_all(&payload)?;
    } else {
        error!("Failed to decode message.");
    }
    Ok(())
}
