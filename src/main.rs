use clap::Parser;
use cli::Commands;
use config::Config;
use lazy_static::lazy_static;
use vm::VM;

mod builder;
mod cli;
mod complier;
mod config;
mod core;
mod vm;

lazy_static! {
    #[derive(Debug)]
    static ref CONFIGURE: Config = Config::init();
}

#[tokio::main]
async fn main() {
    let fog = cli::Cli::parse();
    match &fog.commands {
        Commands::Run { file, debug } => {
            let ir = if let Some(file) = file {
                complier::complie_file(file, None).into()
            } else {
                complier::complie("src")
            };
            if *debug {
                println!("{:#?}", ir.blocks.clone());
            }
            let mut vm = VM::new(ir);
            vm.execute().await;
        }
        Commands::New { name } => {
            builder::init_project(name);
        }
    }
}
