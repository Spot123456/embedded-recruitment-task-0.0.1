
use embedded_recruitment_task::{
    message::{client_message, server_message, AddRequest, EchoMessage},
    server::Server,
};
use std::{sync::Arc, thread};

mod client;

fn setup_server_thread(server: Arc<Server>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        server.run().expect("Server encountered an error");
    })
}

fn create_server() -> Arc<Server> {
    Arc::new(Server::new("localhost:8080").expect("Failed to start server"))
}

#[test]
fn test_client_echo_message() {
    let server = create_server();
    let handle = setup_server_thread(server.clone());

    let mut client = client::Client::new("localhost", 8080, 1000);
    assert!(client.connect().is_ok(), "Failed to connect to the server");

    let mut echo_message = EchoMessage::default();
    echo_message.content = "Hello, World!".to_string();
    let message = client_message::Message::EchoMessage(echo_message.clone());

    assert!(client.send(message).is_ok(), "Failed to send message");

    let response = client.receive();
    assert!(response.is_ok(), "Failed to receive response for EchoMessage");

    match response.unwrap().message {
        Some(server_message::Message::EchoMessage(echo)) => {
            assert_eq!(echo.content, echo_message.content, "Echoed message content does not match");
        }
        _ => panic!("Expected EchoMessage, but received a different message"),
    }

    client.disconnect().unwrap();
    server.stop();
    handle.join().unwrap();
}
