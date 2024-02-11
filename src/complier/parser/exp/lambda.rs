use std::sync::Arc;

use crate::{
    complier::parser::Parser,
    core::{
        block::{Block, BlockType},
        bytecode::ByteCode,
        token::TokenVal,
        value::Value,
    },
};

impl Parser {
    // ()=>{}
    pub fn load_lambda(&mut self) -> ByteCode {
        self.assert_next(TokenVal::ParL);
        self.assert_next(TokenVal::ParR);
        self.assert_next(TokenVal::RightArror);
        let mut codes = self.parse_bucket();
        let mut block = Block::new("".to_string(), Default::default(), BlockType::Fn, vec![]);
        block.byte_codes.append(&mut codes);
        ByteCode::LoadValue(Value::Function(
            crate::core::value::Function::NormalFunction(Arc::new(block)),
        ))
    }
}
