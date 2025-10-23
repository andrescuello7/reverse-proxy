
pub mod deser;
use serde_derive::Deserialize;
use tokio::net::TcpListener;

#[derive(Debug, Deserialize)]
pub struct ProxyConfig {
    pub server: Vec<Server>,
    pub forward: Vec<Forward>,
    pub backends: Vec<Backends>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub listen: Vec<String>,
    #[serde(skip)]
    pub listener: Option<TcpListener>,
}

#[derive(Debug, Deserialize)]
pub struct Forward {
    pub algorithm: String,
}

#[derive(Debug, Deserialize)]
pub struct Backends {
    pub backends: Vec<Backend>,
}

#[derive(Debug, Deserialize)]
pub struct Backend {
    pub address: String,
    pub weight: i16,
}
