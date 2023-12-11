mod lexer;
mod parser;

use crate::core::ir::IR;

use self::{lexer::Lexer, parser::Parser};
use std::fs::File;

pub struct Complier {}

impl Complier {
    pub fn complie(input: File) -> IR {
        let stream = Lexer::from(input).into_token_stream();
        Parser::from(stream).into_ir()
    }
}
