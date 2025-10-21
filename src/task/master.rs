use crate::{
    config::ProxyConfig,
    task::server::{Server, Worker},
};
use std::process::exit;

// Func Master [run] principal init for the program
// This is principal Worker and it has Child process
//
//                     +-----------+
//                     |   Master  |
//                     +-----------+
//                           |
//                           v
//                     +-----------+
//                     |   Server  |
//                     +-----------+
//                           |
//                           v
//   +-----------+     +-----------+     +-----------+
//   |   Worker  | --> |   Worker  | --> |   Worker  |
//   +-----------+     +-----------+     +-----------+

pub struct Master {
    server: Server,
}

impl Master {
    /// See [`Server::init`] for more details.
    pub fn init(config: ProxyConfig) -> Result<Self, crate::Error> {
        let mut server: Server = {
            Server {
                listen: config.server[0].listen[0].clone(),
                listener: None,
                workers: Vec::new(),
            }
        };

        for backend in &config.backends[0].backends {
            let worker = {
                Worker {
                    address: backend.address.clone(),
                    weight: backend.weight,
                    socket: None,
                }
            };
            server.workers.push(worker);
        }

        Ok(Self { server })
    }

    pub async fn run(&mut self) -> Result<(), crate::Error> {
        // TODO: Run single server for moment
        if let Err(e) = self.server.create_server().await {
            eprintln!("failed creation of server: {}", e);
            exit(1);
        };

        if let Err(e) = self.server.spawn_backend().await {
            eprintln!("failed spawning worker: {}", e);
            exit(1);
        };

        let workers_len = self.server.workers.len();

        for _ in 0..workers_len {
            if let Err(e) = self.server.socket_listener().await {
                eprintln!("failed listener child worker > socket: {}", e);
                exit(1);
            };
        }
        Ok(())
    }
}
