mod builder;
mod cli;
mod complier;
mod config;
mod core;
mod vm;

use anyhow::Result;
use config::Config;
use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIGURE: Config = Config::init().unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await
}
