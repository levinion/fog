pub mod block;
pub mod ir;
mod lex;
mod parse;

use self::{ir::IR, lex::Lex, parse::Parser};
use std::fs::File;

pub struct Complier {}

impl Complier {
    pub fn complie(input: File) -> IR {
        let stream = Lex::from(input).into_token_stream();
        Parser::from(stream).into_ir()
    }
}
