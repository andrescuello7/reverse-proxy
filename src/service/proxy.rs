use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{ TcpStream };

pub struct Proxy {}

impl Proxy {
    pub async fn spawn_backends() -> Result<String, Box<dyn std::error::Error>> {
        // Conexión TCP al host y puerto
        let mut stream = TcpStream::connect("localhost:3000").await?;
        println!("[+] Connect to localhost:3000");

        // Petición HTTP simple
        let request = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
        stream.write_all(request).await?;

        // Leer respuesta
        let mut response = Vec::new();
        stream.read_to_end(&mut response).await?;

        let answer = String::from_utf8_lossy(&response);
        Ok(answer.to_string())
    }
}
