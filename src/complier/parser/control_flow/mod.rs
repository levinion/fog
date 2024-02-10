use crate::core::{bytecode::ByteCode, token::TokenVal};

use super::Parser;

impl Parser {
    /// enter if block
    /// [if exp {@println("hello");}]
    pub fn parse_if(&mut self) -> Vec<ByteCode> {
        let mut codes = vec![];
        // if
        self.assert_next(TokenVal::If);
        let mut exp = self.load_exp();
        let mut if_bucket = self.parse_bucket();

        codes.append(&mut exp);
        codes.push(ByteCode::JumpIfFalse(if_bucket.len() as isize + 1));
        codes.append(&mut if_bucket);

        // else
        if let TokenVal::Else = self.stream.look_ahead(1).0.val {
            self.stream.next();
            let mut else_bucket = self.parse_bucket();
            codes.push(ByteCode::Jump(else_bucket.len() as isize));
            codes.append(&mut else_bucket);
        }

        codes
    }
}
