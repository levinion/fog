mod builder;
mod cli;
mod complier;
mod config;
mod core;
mod vm;

use anyhow::Result;
use config::Config;
use lazy_static::lazy_static;
use tokio::sync::Mutex;
use vm::Vm;

lazy_static! {
    static ref CONFIGURE: Config = Config::init().unwrap();
    static ref VM: Mutex<Vm> = Mutex::new(Vm::new());
}

#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await
}
