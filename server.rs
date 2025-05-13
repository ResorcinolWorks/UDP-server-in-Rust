use std::net::UdpSocket;
use std::str;
use std::env;

fn main() -> std::io::Result<()> {
    // Get IP and port from environment variables or use defaults
    let ip = env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", ip, port);

    // Create a UDP socket
    let socket = UdpSocket::bind(&addr)?;
    println!("UDP Server listening on {}", addr);

    // Buffer for receiving data
    let mut buf = [0; 1024];

    loop {
        // Receive data from client
        let (amt, src) = socket.recv_from(&mut buf)?;
        
        // Convert received bytes to string
        let received = str::from_utf8(&buf[..amt])
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        println!("Received from {}: {}", src, received);

        // Echo the message back to the client
        socket.send_to(&buf[..amt], &src)?;
        println!("Echoed back to {}", src);
    }
} 