use std::fs::File;

use clap::Parser;
use lex::Lex;
use vm::VM;

mod core;
mod lex;
mod parse;
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

fn main() {
    let cli = Cli::parse();

    let file = File::open(&cli.input).unwrap();
    let stream = Lex::from(file).into_token_stream();
    let ir = parse::Parser::from(stream).into_ir();
    if cli.debug {
        ir.debug();
    }
    let mut vm = VM::new();
    vm.execute(ir);
}
