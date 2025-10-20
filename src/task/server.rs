use std::process::exit;

use tokio::io::{AsyncWriteExt};
use tokio::net::TcpListener;

use crate::config::Server;
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

            tokio::spawn(async move {
                let raw_request = Proxy::read_full_http_request(&mut socket)
                    .await
                    .expect("Failed to read request");
                println!("--- RAW REQUEST [{}] ---\n{:?}", addr, raw_request);

                let res_proxy_to_backend = Proxy::proxy_to_backend(raw_request)
                    .await
                    .expect("Failed to read request");

                socket
                    .write_all(res_proxy_to_backend.as_bytes())
                    .await
                    .expect("Failed to read request");
            });
        }
    }
}
