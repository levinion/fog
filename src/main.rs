#![feature(lazy_cell)]

mod builder;
mod cli;
mod complier;
mod config;
mod core;
mod vm;

use std::sync::{LazyLock, OnceLock};

use anyhow::Result;
use config::Config;
use vm::Vm;

static CONFIGURE: LazyLock<Config> = LazyLock::new(|| Config::init().unwrap());
static VM: OnceLock<Vm> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    cli::run().await
}
