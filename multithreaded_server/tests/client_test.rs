use multithreaded_server::{
    messages::{client_message, server_message, EchoMessage},
    server::Server,
};
use std::{sync::Arc, thread, time::Duration};
use multithreaded_server::client;
use log::{info, error};

fn setup_server_thread(server: Arc<Server>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        info!("Starting server thread...");
        server.run().expect("Server encountered an error");
    })
}

fn create_server() -> Arc<Server> {
    info!("Creating server...");
    Arc::new(Server::new("localhost:8080").expect("Failed to start server"))
}

#[test]
fn test_client_echo_message() {
    env_logger::init(); // Initialize the logger for tests
    info!("Starting test_client_echo_message...");

    // Create and start the server
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    // Wait for the server to start listening
    thread::sleep(Duration::from_secs(1));

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8080, 1000);
    info!("Connecting client to server...");
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    // Prepare and send the EchoMessage
    let mut echo_message = EchoMessage::default();
    echo_message.content = "Hello, World!".to_string();
    let message = client_message::Message::EchoMessage(echo_message.clone());

    info!("Sending EchoMessage to server...");
    assert!(client.send(message).is_ok(), "Failed to send message");

    // Wait for the server's response
    info!("Waiting for response from server...");
    let response = client.receive();
    assert!(response.is_ok(), "Failed to receive response for EchoMessage");

    // Verify the response
    match response.unwrap().message {
        Some(server_message::Message::EchoMessage(echo)) => {
            info!("Received EchoMessage from server: {}", echo.content);
            assert_eq!(echo.content, echo_message.content, "Echoed message content does not match");
        }
        _ => panic!("Expected EchoMessage, but received a different message"),
    }

    // Disconnect the client
    info!("Disconnecting client...");
    client.disconnect().unwrap();

    // Stop the server
    info!("Stopping server...");
    server.stop();

    // Wait for the server thread to finish
    info!("Joining server thread...");
    handle.join().unwrap();

    info!("Test completed successfully!");
}