use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    // Create a UDP socket
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    println!("UDP Server listening on 127.0.0.1:8080");

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