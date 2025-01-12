use std::{
    net::{TcpListener, TcpStream},
    io::{self, Read, Write},
    sync::{Arc, Mutex},
};
use threadpool::ThreadPool;
use log::{info, error};
use prost::Message;
use crate::messages::{server_message, ServerMessage, AddResponse};

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

    info!("Received {} bytes from client.", bytes_read);

    // Decode the raw bytes into `ClientMessage`
    if let Ok(client_message) = crate::messages::ClientMessage::decode(&buffer[..bytes_read]) {
        info!("Decoded ClientMessage: {:?}", client_message);

        match client_message.message {
            Some(crate::messages::client_message::Message::EchoMessage(echo)) => {
                info!("Received EchoMessage: {}", echo.content);
                let response = ServerMessage {
                    message: Some(server_message::Message::EchoMessage(echo)),
                };
                let mut payload = Vec::new();
                response.encode(&mut payload).unwrap();
                stream.write_all(&payload)?;
                info!("Sent EchoMessage response to client.");
            }
            Some(crate::messages::client_message::Message::AddRequest(add)) => {
                info!("Received AddRequest: {} + {}", add.a, add.b);
                let result = add.a + add.b;
                let response = ServerMessage {
                    message: Some(server_message::Message::AddResponse(
                        AddResponse { result },
                    )),
                };
                let mut payload = Vec::new();
                response.encode(&mut payload).unwrap();
                stream.write_all(&payload)?;
                info!("Sent AddResponse to client.");
            }
            None => {
                error!("Received a ClientMessage with no content.");
            }
        }
    } else {
        error!("Failed to decode message.");
    }

    Ok(())
}