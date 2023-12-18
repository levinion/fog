mod builder;
mod cli;
mod complier;
mod config;
mod core;
mod vm;

use config::Config;
use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIGURE: Config = Config::init();
}

#[tokio::main]
async fn main() {
    cli::run().await;
}
