use std::process::exit;

use tokio::net::{TcpListener, TcpStream};

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

pub struct Server {
    pub listen: String,
    pub listener: Option<TcpListener>,
    pub workers: Vec<Worker>,
}

pub struct Worker {
    pub address: String,
    pub weight: i16,
    pub socket: Option<TcpStream>,
}

impl Server {
    pub async fn create_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Server run in this line > server listen to localhost:9000
        let server = &self.listen;
        let listener = match TcpListener::bind(server).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed creation of server or principal worker: {}", e);
                exit(1)
            }
        };
        println!("[+] Listening on http://{}", server);

        self.listener = Some(listener);
        Ok(())
    }

    pub async fn spawn_backend(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.workers[0].socket = Some(
            Proxy::spawn_backend()
                .await
                .expect("Failed creation socket worker"),
        );
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
            tokio::spawn(async move {
                let raw_request = Proxy::read_full_http_request(&mut socket)
                    .await
                    .expect("Failed to read request");

                println!("--- RAW REQUEST [{}] ---\n{:?}", addr, raw_request);

                
                // if let Some(socket) = &mut self.workers[0].socket {
                //     socket
                //         .write_all(raw_request.as_bytes())
                //         .await
                //         .expect("Failed to write request to worker");
                // } else {
                //     eprintln!("Worker socket not initialized");
                // }


                // let worker = self.workers[0].socket;
                // worker
                //     .write_all(raw_request.as_bytes())
                //     .await
                //     .expect("Failed to read request");


                // let mut response = Vec::new();
                // worker
                //     .read_to_end(&mut response)
                //     .await
                //     .expect("Failed to read request");


                // let answer = String::from_utf8_lossy(&response).to_string();
                // socket
                //     .write_all(answer.as_bytes())
                //     .await
                //     .expect("Failed to read request");
            });
        }
    }
}
