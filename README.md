# embedded-recruitment-task-0.0.1
multi-threaded server


# Multithreaded Rust Server

This project is a multithreaded server implemented in Rust using the `threadpool` crate. It includes client and server components as well as a test suite to verify functionality.

## Project Structure
```
.
├── Cargo.toml        # Rust dependencies and configuration
├── src
│   ├── main.rs       # Main entry point of the server
│   ├── server.rs     # Server implementation
│   └── client.rs     # Client implementation
└── tests
    └── client_test.rs # Test suite for the server
```

## Prerequisites
- Rust (latest stable version recommended)
- Cargo (Rust package manager)
- for understanding RUST i recommended for you this playlist : https://www.youtube.com/watch?v=lzKeecy4OmQ

## How to Run the Project

### 1. Clone the Repository
If you have the project as a ZIP file, unzip it. Otherwise, clone the repository (if applicable).

```bash
git clone https://github.com/Spot123456/embedded-recruitment-task-0.0.1/multithreaded_server.git
cd multithreaded_server
```

### 2. Build the Project
Run the following command to build the project:

```bash
cargo build
```

### 3. Run the Server
Start the server by running the following command:

```bash
cargo run
```

The server will start listening on `localhost:8080`.

### 4. Run the Test Suite
To verify that the server is working correctly, run the test suite:

```bash
cargo test
```

### 5. Connecting a Client
The client implementation in `client.rs` can be used to connect to the server. Modify the client code as needed to send messages to the server.

### Notes
- The server is implemented using a thread pool to efficiently handle multiple client connections.
- The test suite includes various tests to verify server functionality and client-server communication.

## Dependencies
- `log`: For logging messages.
- `env_logger`: For initializing the logger.
- `prost`: For encoding and decoding messages using Protocol Buffers.
- `threadpool`: For creating a thread pool to handle client connections.

