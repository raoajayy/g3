use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting minimal ICAP test server...");
    
    let listener = TcpListener::bind("0.0.0.0:1346").await?;
    println!("Server listening on 0.0.0.0:1346");
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);
        
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            
            match socket.read(&mut buffer).await {
                Ok(n) => {
                    println!("Received {} bytes: {}", n, String::from_utf8_lossy(&buffer[..n]));
                    
                    let response = b"ICAP/1.0 200 OK\r\nISTag: \"test-1.0\"\r\n\r\n";
                    if let Err(e) = socket.write_all(response).await {
                        eprintln!("Failed to write response: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        });
    }
}
