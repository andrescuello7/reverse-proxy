use std::process::exit;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;

use crate::http::err_enum::Errs;
use crate::service::proxy::Proxy;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

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
    pub socket: Option<Arc<Mutex<TcpStream>>>,
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

    // pub async fn spawn_backend(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    //     self.workers[0].socket = Some(Arc::new(Mutex::new(
    //         Proxy::spawn_backend()
    //             .await
    //             .expect("Failed creation socket worker"),
    //     )));
    //     Ok(())
    // }

    pub async fn socket_listener(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = match &mut self.listener {
            Some(l) => l,
            None => {
                eprintln!("no listener initialized");
                return Ok(());
            }
        };

        loop {
            let (mut socket, addr) = listener.accept().await?;

            let new_worker = match Proxy::spawn_backend().await {
                Ok(w) => Arc::new(Mutex::new(w)),
                Err(_) => {
                    println!("Failed to write request");
                    
                    socket
                        .write_all(Errs::Err502("").message().as_bytes())
                        .await?;
                    return Ok(());
                }
            };
            self.workers[0].socket = Some(new_worker);

            // Clone the Arc for each spawned task
            let worker_socket_clone = self.workers[0].socket.clone();

            tokio::spawn(async move {
                let raw_request = Proxy::read_full_http_request(&mut socket)
                    .await
                    .expect("Failed to read request");

                println!("--- RAW REQUEST [{}] ---\n{:?}", addr, raw_request);

                if let Some(worker_socket) = worker_socket_clone {
                    let mut worker = worker_socket.lock().await;

                    match worker.write_all(raw_request.as_bytes()).await {
                        Ok(_) => {
                            let mut response = Vec::new();
                            worker
                                .read_to_end(&mut response)
                                .await
                                .expect("Failed to read request");

                            let answer = String::from_utf8_lossy(&response).to_string();
                            socket
                                .write_all(answer.as_bytes())
                                .await
                                .expect("Failed to read request");
                        }
                        Err(_) => {
                            println!("Failed to write request");

                            socket
                                .write_all(Errs::Err400("").message().as_bytes())
                                .await
                                .expect("Failed to read request")
                        }
                    }
                } else {
                    eprintln!("No socket available for worker 0");
                }
            });
        }
    }
}
