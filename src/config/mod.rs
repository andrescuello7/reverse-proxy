use serde_derive::Deserialize;
pub mod deser;

// Top level struct to hold the TOML data.
#[derive(Deserialize)]
pub struct ProxyEnvirements {
    pub(crate) server: Listen,
    pub(crate) services: Services,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
pub struct Listen {
    pub(crate) listen: Vec<Server>,
}

#[derive(Deserialize)]
pub struct Services {
    pub(crate) backends: Vec<Server>,
}

#[derive(Deserialize)]
pub struct Server {
    /// This is of direction of server >> IP:PORT.
    pub(crate) address: String,
    pub(crate) weight: i16,
}