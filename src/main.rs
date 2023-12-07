use std::{env, fs::File};

use lex::Lex;
use parse::Parser;
use vm::VM;

mod bytecode;
mod lex;
mod parse;
mod token;
mod variable;
mod vm;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid arguments!")
    }
    let file = File::open(&args[1]).unwrap();
    let lex = Lex::from(file);
    let mut parser = Parser::from(lex);
    parser.parse();
    let mut vm = VM::new();
    vm.execute(parser);
}
