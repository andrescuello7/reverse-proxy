mod task;
mod config;
mod service;

use hyper::Error;
use task::master::Master;
use config::deser::{read_config, parser_data};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = parser_data(read_config());
    Master::init(config)?.run().await?;
    Ok(())
}
