use std::process::exit;
use crate::config::{Backends, ProxyConfig, Server};

// Func Master [run] principal init for the program
// This is principal Worker and it has Child process
// 
//                     +-----------+
//                     |   Master  |
//                     +-----------+
//                           |
//                           v
//   +-----------+     +-----------+     +-----------+
//   |   Worker  | --> |   Worker  | --> |   Worker  |
//   +-----------+     +-----------+     +-----------+

pub struct Master {
    // All the servers that the master has spawned.
    servers: Vec<Server>,
    backends: Vec<Backends>,
}

impl Master {
    /// See [`Server::init`] for more details.
    pub fn init(config: ProxyConfig) -> Result<Self, crate::Error> {
        let mut servers: Vec<Server> = Vec::new();
        let mut backends: Vec<Backends> = Vec::new();

        for server in config.server{
            servers.push(server);
        }
        for backend in config.backends{
            backends.push(backend);
        }

        Ok(Self {
            servers,
            backends
        })
    }

    pub async fn run(&mut self) -> Result<(), crate::Error> {
        // TODO: Run single server for moment
        let server = &mut self.servers[0];

        if let Err(e) = server.create_server().await {
            eprintln!("failed creation of server or principal worker: {}", e);
            exit(1);
        };

        if let Err(e) = server.socket_listener().await {
            eprintln!("failed listener child worker > socket: {}", e);
            exit(1);
        };
        
        Ok(())
    }
}
