# UDP Server-Client Implementation in Rust

A simple UDP-based client-server communication system implemented in Rust. This project demonstrates basic UDP socket programming and network communication.

## Features

- UDP server implementation
- UDP client implementation
- Simple message exchange protocol
- Basic error handling

## Prerequisites

- Rust compiler and Cargo (Rust's package manager)
- To install Rust, visit https://rustup.rs/

## Building the Project

1. Compile the server:
```bash
rustc server.rs
```

2. Compile the client:
```bash
rustc client.rs
```

## Running the Application

1. Start the server in one terminal:
```bash
./server
```

2. Start the client in another terminal:
```bash
./client
```

## Usage

1. The server will start listening on `127.0.0.1:8080`
2. The client will connect to the server automatically
3. Type messages in the client terminal to send them to the server
4. The server will echo back any received messages
5. Type 'quit' in the client to exit

## Error Handling

The implementation includes basic error handling for:
- Socket creation and binding
- Data transmission
- UTF-8 encoding/decoding
- User input processing

## Notes

- The server runs on localhost (127.0.0.1) for demonstration purposes
- The server will continue running until manually terminated
- The client can be stopped by typing 'quit' 