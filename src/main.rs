use clap::Parser;
use cli::Commands;
use vm::VM;

mod builder;
mod cli;
mod complier;
mod core;
mod vm;

#[tokio::main]
async fn main() {
    let fog = cli::Cli::parse();
    match &fog.commands {
        Commands::Run { file, debug } => {
            let ir = if let Some(file) = file {
                complier::complie_file(file).into()
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
            let builder = builder::Builder::new();
            builder.init_project(name);
        }
    }
}
