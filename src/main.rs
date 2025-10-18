mod config;
mod service;
mod task;

use config::deser::to_toml;
use hyper::Error;
use task::master::Master;
use crate::config::ProxyEnvirements;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: ProxyEnvirements = to_toml();
    Master::init(config)?.run().await?;

    Ok(())
}
