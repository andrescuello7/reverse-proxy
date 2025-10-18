use crate::config::{ProxyEnvirements, Server};

pub struct Master {
    // All the servers that the master has spawned.
    servers: Vec<Server>,
    backends: Vec<Server>,
}

impl Master {
    /// See [`Server::init`] for more details.
    pub fn init(config: ProxyEnvirements) -> Result<Self, crate::Error> {
        let mut servers: Vec<Server> = Vec::new();
        let mut backends: Vec<Server> = Vec::new();

        for server in config.server.listen{
            servers.push(server);
        }
        for backend in config.services.backends{
            backends.push(backend);
        }

        Ok(Self {
            servers,
            backends
        })
    }

    /// All the servers are put into `listen` mode and they start accepting connections.
    pub async fn run(self) -> Result<(), crate::Error> {
        for server in self.servers{
            let _ = server.run().await.inspect_err(|e| eprintln!("failed creation of server: {e}"));
        }
        Ok(())
    }
}
