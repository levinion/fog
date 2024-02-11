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
        codes.push(ByteCode::JumpIfFalse(if_bucket.len() as isize));
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

    pub fn parse_for(&mut self) -> Vec<ByteCode> {
        let mut codes = vec![];
        self.assert_next(TokenVal::For);
        if let TokenVal::CurlyL = self.stream.look_ahead(1).0.val {
            // loop
            let mut bucket = self.parse_bucket();
            let step = bucket.len() as isize;
            codes.append(&mut bucket);
            codes.push(ByteCode::Jump(-(step + 1)));
        } else {
            let mut exp1 = self.load_exp();
            if let TokenVal::CurlyL = self.stream.look_ahead(1).0.val {
                // while
                let mut bucket = self.parse_bucket();
                let bucket_len = bucket.len() as isize;
                let exp1_len = exp1.len() as isize;
                codes.append(&mut exp1);
                codes.push(ByteCode::JumpIfFalse(bucket_len + 1));
                codes.append(&mut bucket);
                codes.push(ByteCode::Jump(-(bucket_len + exp1_len + 2)));
            } else {
                // for in
                todo!();
            }
        }
        codes
    }

    // return v;
    pub fn parse_return(&mut self) -> Vec<ByteCode> {
        let mut codes = vec![];
        self.assert_next(TokenVal::Return);
        codes.append(&mut self.load_exp());
        codes.push(ByteCode::Return);
        self.assert_next(TokenVal::SemiColon);
        codes
    }
}
