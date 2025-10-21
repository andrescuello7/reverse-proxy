use tokio::io::{AsyncReadExt};
use tokio::net::TcpStream;

pub struct Proxy {}

impl Proxy {
    pub async fn read_full_http_request(
        req_socket: &mut TcpStream,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        let mut temp = [0; 1024];

        // Leemos hasta que encontremos el final de headers (\r\n\r\n)
        loop {
            let n = req_socket.read(&mut temp).await?;
            if n == 0 {
                break;
            }
            buffer.extend_from_slice(&temp[..n]);
            if buffer.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
        }

        // Convertimos a String para procesar headers
        let headers_text = String::from_utf8_lossy(&buffer);
        let mut content_length = 0usize;

        // Si hay Content-Length, lo parseamos
        for line in headers_text.lines() {
            if line.to_lowercase().starts_with("content-length:") {
                if let Some(value) = line.split(':').nth(1) {
                    content_length = value.trim().parse::<usize>().unwrap_or(0);
                }
            }
        }

        // Si hay body, lo leemos completo
        let mut total_body = Vec::new();
        while total_body.len() < content_length {
            let n = req_socket.read(&mut temp).await?;
            if n == 0 {
                break;
            }
            total_body.extend_from_slice(&temp[..n]);
        }

        // Unimos headers + body
        let mut full_request = buffer;
        full_request.extend_from_slice(&total_body);

        Ok(String::from_utf8_lossy(&full_request).to_string())
    }

    pub async fn spawn_backend() -> Result<TcpStream, Box<dyn std::error::Error>> {
        let backend = TcpStream::connect("localhost:3000").await.inspect_err(|e| eprintln!("Error TCPStream Geneteation Worker {}", e));
        println!("[+] Connected to backend localhost:3000");
        Ok(backend?)
    }
}
