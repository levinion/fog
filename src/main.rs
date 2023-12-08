use std::{env, fs::File};

use lex::Lex;
use parse::Parser;
use vm::VM;

mod core;
mod lex;
mod parse;
mod vm;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid arguments!")
    }
    let file = File::open(&args[1]).unwrap();
    let stream = Lex::from(file).into_token_stream();
    let ir = Parser::from(stream).into_ir();
    ir.debug();
    let mut vm = VM::new();
    vm.execute(ir);
}
