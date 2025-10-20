use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Proxy {}

impl Proxy {
    pub async fn read_full_http_request(
        socket: &mut TcpStream,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        let mut temp = [0; 1024];

        // Leemos hasta que encontremos el final de headers (\r\n\r\n)
        loop {
            let n = socket.read(&mut temp).await?;
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
            let n = socket.read(&mut temp).await?;
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

    /// 🔁 Reenvía la request completa al backend en localhost:3000
    pub async fn proxy_to_backend(raw_request: String) -> Result<String, Box<dyn std::error::Error>> {
        let mut backend = TcpStream::connect("localhost:3000").await?;
        println!("[+] Connected to backend localhost:3000");

        // Escribimos la request tal como llegó
        backend.write_all(raw_request.as_bytes()).await?;

        // Leemos toda la respuesta del backend
        let mut response = Vec::new();
        backend.read_to_end(&mut response).await?;

        Ok(String::from_utf8_lossy(&response).to_string())
    }
}
