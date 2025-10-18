use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::config::{Server};
use crate::service::proxy::Proxy;

impl Server {
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        // Server run in this line >> server listen to localhost:9000
        let listener = TcpListener::bind(self.address.clone()).await?;
        println!("[+] Listening on http://{}", self.address);

        loop {
            // 
            // New Connection for sockets for Protocol [`TCP/HTTP`]
            // From user or principal client we have Head Cusmom X-Forwarded-For With info
            // We need parser this data and save or added to array of socket 
            // With Struct for used from responses
            // This part of comminication with services
            //  
            //          +-----------+
            //          |   Client  | 
            //          +-----------+    
            //                |
            //                v                  
            //          +-----------+       +----------+       +----------+
            //          | Connected | --->  |  Socket  | --->  |   Proxy  |
            //          +-----------+       +----------+       +----------+
            // 
            // 
            let (mut socket, addr) = listener.accept().await?;
            println!("[] new request from: http://{}", addr);

            println!("[] socket: http://{:?}", socket);

            // Try connecting with backends setting for default >> https://tcp.azure.net:8080 | https://tcp.azure.net:80
            let res_backend = Proxy::spawn_backends().await?;
            println!("Respuesta de la pegada a la API {:?}", res_backend);

            // Crea una tarea independiente para manejar el cliente
            tokio::spawn(async move {
                let mut buffer = [0; 1024];

                loop {
                    let n = match socket.read(&mut buffer).await {
                        Ok(0) => {
                            println!("Cliente desconectado: {}", addr);
                            return;
                        }
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("Error leyendo de {}: {:?}", addr, e);
                            return;
                        }
                    };

                    // Envía el mismo mensaje de vuelta (eco)
                    if let Err(e) = socket.write_all(&buffer[..n]).await {
                        eprintln!("Error enviando a {}: {:?}", addr, e);
                        return;
                    }
                }
            });
        }
    }
}
