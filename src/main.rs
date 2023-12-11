use std::fs::File;

use clap::Parser;
use vm::VM;

mod complier;
mod core;
mod vm;

#[derive(clap::Parser)]
#[command(name = "fog")]
#[command(author = "levinion <levinnion@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "A simple language", long_about = None)]
pub struct Cli {
    input: String,
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let file = File::open(&cli.input).unwrap();
    let ir = complier::Complier::complie(file);
    if cli.debug {
        for block in &ir.blocks {
            println!("{:#?}", block);
        }
    }
    let mut vm = VM::new(ir);
    vm.execute().await;
}
