use std::sync::Arc;
use std::{process::exit, time::Duration};
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use tokio::time::timeout;

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
//          +-----------+
//          |  PWorker  |
//          +-----------+
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
                eprintln!("[x] Failed creation of server or principal worker: {}", e);
                exit(1)
            }
        };
        println!("[+] Listening on http://{}", server);
        self.listener = Some(listener);
        Ok(())
    }

    pub async fn spawn_backend(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let backend = &mut self.workers[index];
        let create_socket: Arc<Mutex<TcpStream>> = match Proxy::spawn_backend(backend).await {
            Ok(w) => Arc::new(Mutex::new(w)),
            Err(_) => {
                println!("Failed to write request");
                return Ok(());
            }
        };
        backend.socket = Some(create_socket);
        Ok(())
    }

    pub async fn socket_listener(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let listener = match &mut self.listener {
            Some(l) => l,
            None => {
                eprintln!("no listener initialized");
                return Ok(());
            }
        };

        loop {
            let (mut master, addr) = listener.accept().await?;
            let backend = self.workers[index].socket.clone();

            tokio::spawn(async move {
                let raw_request = Proxy::read_full_http_request(&mut master)
                    .await
                    .expect("Failed to read request");

                println!("-- REQUEST [{}]", addr);

                if let Some(mx_backend_socket) = backend {
                    let mut be_socket = mx_backend_socket.lock().await;

                    match be_socket.write_all(raw_request.as_bytes()).await {
                        Ok(_) => {
                            let mut response = Vec::new();

                            let _ =
                                timeout(Duration::from_secs(1), be_socket.read_to_end(&mut response))
                                    .await;

                            let answer = String::from_utf8_lossy(&response).to_string();

                            master
                                .write_all(answer.as_bytes())
                                .await
                                .expect("Failed to read request");
                        }
                        Err(_) => {
                            println!("Failed to write request");

                            master
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
