use std::process::exit;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::config::{Server};
use crate::service::proxy::Proxy;

// 
// New Connection for sockets for Protocol [`TCP/HTTP`]
// From user or principal client we have Head Cusmom X-Forwarded-For With info
// We need parser this data and save or added to array of socket 
// With Struct for used from responses or answers
// This part of comminication with services
//  
//          +------------+
//          |   PWorker  | 
//          +------------+    
//                |
//                v                  
//          +-----------+       +----------+       +----------+
//          |  Connect  | --->  |  Socket  | --->  |   Proxy  |
//          +-----------+       +----------+       +----------+
// 
// 
impl Server {
    pub async fn create_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Server run in this line > server listen to localhost:9000
        let master = &self.listen[0];
        let listener = match TcpListener::bind(master).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed creation of server or principal worker: {}", e);
                exit(1)
            }
        };
        println!("[+] Listening on http://{}", master);

        self.listener = Some(listener);
        Ok(())
    }

    pub async fn socket_listener(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // necesitamos el listener que creamos antes
        let listener = match &mut self.listener {
            Some(l) => l,
            None => {
                eprintln!("no listener initialized");
                return Ok(());
            }
        };

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("[] new request from: http://{}", addr);
            println!("[] socket: http://{:?}", socket);

            // Try connecting with backends setting for default >> https://tcp.azure.net:8080 | https://tcp.azure.net:80
            let res_backend = Proxy::spawn_backends().await?;
            println!("Respuesta de la pegada a la API {:?}", res_backend);

            // Make task for client...
            tokio::spawn(async move {
                let mut buffer = [0; 1024];

                loop {
                    let n = match socket.read(&mut buffer).await {
                        Ok(0) => {
                            println!("Client undefined: {}", addr);
                            return;
                        }
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("Error read from {}: {:?}", addr, e);
                            return;
                        }
                    };

                    // Envía el mismo mensaje de vuelta (eco)
                    if let Err(e) = socket.write_all(&buffer[..n]).await {
                        eprintln!("Error sending to {}: {:?}", addr, e);
                        return;
                    }
                }
            });
        }
    }
}
