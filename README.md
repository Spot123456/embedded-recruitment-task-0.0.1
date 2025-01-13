
# 🚀 Multithreaded Rust Server

A high-performance, multithreaded server implemented in Rust using the `threadpool` crate. This project demonstrates efficient client-server communication using Protocol Buffers and includes a server, client, and test suite.

---

## 📂 Project Structure

```plaintext
.
├── Cargo.toml            # Rust dependencies and configuration
├── build.rs              # Build script for generating Protobuf code
├── proto/                # Protocol Buffers definitions
│   └── messages.proto
├── src/
│   ├── main.rs           # Main entry point of the server
│   ├── lib.rs            # Library module exposing server, client, and messages
│   ├── server.rs         # Server implementation
│   ├── client.rs         # Client implementation
│   └── messages.rs       # Generated Protobuf code
└── tests/
    └── client_test.rs    # Test suite for the server
```

---

## 🛠️ Prerequisites

Before running the project, ensure you have the following installed:

- **Rust**: Latest stable version. Install via [rustup.rs](https://rustup.rs/).
- **Cargo**: Rust's package manager (included with Rust).
- **Protocol Buffers**: Install the `protoc` compiler. Follow the [official guide](https://grpc.io/docs/protoc-installation/).

---

## 🚀 Getting Started

### 1. Clone the Repository

Clone the repository to your local machine:

```bash
git clone https://github.com/Spot123456/embedded-recruitment-task-0.0.1/multithreaded_server.git
cd multithreaded_server
```

---

### 2. Build the Project

Compile the project using Cargo:

```bash
cargo build
```

---

### 3. Run the Server

Start the server:

```bash
cargo run
```

The server will start listening on `localhost:8080`. You should see logs like:

```plaintext
[INFO] Server is running on localhost:8080
```

---

### 4. Run the Test Suite

To verify the server's functionality, run the test suite:

```bash
cargo test -- --nocapture
```

The `--nocapture` flag ensures that test logs are displayed in the terminal. Sample output:

```plaintext
running 1 test
[INFO] Starting test_client_echo_message...
[INFO] Server stopped.
test test_client_echo_message ... ok

test result: ok. 1 passed; 0 failed; 0 ignored
```

---

### 5. Connect a Client

Use the client implementation in `client.rs` to connect to the server. Modify the client as needed to send messages.

---

## 🧩 Features

- **Multithreaded Server**: Uses a thread pool for efficient client connection handling.
- **Protocol Buffers**: Encodes and decodes messages for efficient communication.
- **Test Suite**: Verifies server functionality and client-server communication.
- **Logging**: Implements structured logging with `log` and `env_logger`.

---

## 📦 Dependencies

This project uses the following crates:

- `log`: For logging.
- `env_logger`: Initializes the logger.
- `prost`: Encodes and decodes Protocol Buffer messages.
- `threadpool`: Manages the thread pool for concurrent connections.
- `prost-build`: Generates Rust code from Protocol Buffer definitions.

---

## 📚 Learning Resources

If you're new to Rust, here are some resources:

- [Rust Playlist](https://www.youtube.com/playlist?list=PL5dTjWUk_cPYOPWpUwmOLatvUB9ZwHH7p): Rust Programming Tutorials.
- [Rust Book](https://doc.rust-lang.org/book/): The Rust Programming Language.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/): Rust by Example.

---

## 📝 Notes

- The server handles multiple clients concurrently using a thread pool.
- The test suite includes a basic `test_client_echo_message` to verify the server echoes messages correctly.
- You can extend the server and client to support additional message types and functionality.

---

## 🙌 Contributing

Contributions are welcome! Here's how to get started:

1. Fork the repository.
2. Create a new branch:
   ```bash
   git checkout -b feature/YourFeatureName
   ```
3. Commit your changes:
   ```bash
   git commit -m 'Add some feature'
   ```
4. Push to the branch:
   ```bash
   git push origin feature/YourFeatureName
   ```
5. Open a pull request.

---

## 🛡️ Optional: Add Badges

To add badges for build status, license, etc., include them at the top of the README:

```markdown
![Rust](https://img.shields.io/badge/Rust-v1.60+-orange)

```

---
