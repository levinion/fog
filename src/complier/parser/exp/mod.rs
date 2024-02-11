mod cal;
mod lambda;

use crate::core::{
    bytecode::ByteCode,
    op::{BinaryOP, UnaryOP},
    token::TokenVal,
    value::Value,
};

use super::Parser;

impl Parser {
    pub fn load_exp(&mut self) -> Vec<ByteCode> {
        if let TokenVal::ParL = self.stream.look_ahead(1).0.val {
            return vec![self.load_lambda()];
        }
        let mut codes = vec![];
        let output = self.handle_infix();
        let mut op_count: usize = 0; // count there is how many values on the stack
        for token in output.iter() {
            let val = token.0.val.clone();
            match val {
                TokenVal::Add
                | TokenVal::Sub
                | TokenVal::Mul
                | TokenVal::Div
                | TokenVal::Equal
                | TokenVal::NotEq
                | TokenVal::Greater
                | TokenVal::Less
                | TokenVal::GreEq
                | TokenVal::LesEq => codes.push(self.auto_op(&mut op_count, val)),
                TokenVal::Name(name) => {
                    codes.push(ByteCode::LoadValue(Value::Name(name)));
                    codes.push(ByteCode::LoadName);
                    op_count += 1;
                }
                TokenVal::Int(i) => {
                    codes.push(ByteCode::LoadValue(Value::Int(i)));
                    op_count += 1;
                }
                TokenVal::Float(f) => {
                    codes.push(ByteCode::LoadValue(Value::Float(f)));
                    op_count += 1;
                }
                TokenVal::String(s) => {
                    codes.push(ByteCode::LoadValue(Value::String(s)));
                    op_count += 1;
                }
                TokenVal::Bool(b) => {
                    codes.push(ByteCode::LoadValue(Value::Bool(b)));
                    op_count += 1;
                }
                TokenVal::Type(typ) => {
                    codes.push(ByteCode::LoadValue(Value::Type(typ)));
                    op_count += 1;
                }
                // TODO: Support functions that may have more than 1 argc
                TokenVal::ParR => codes.push(ByteCode::CallFunction(1)),
                token => panic!("unexpected token: {:?}", token),
            }
        }
        codes
    }

    /// load exps separated by comma
    pub fn load_exps(&mut self) -> (Vec<ByteCode>, usize) {
        let mut codes = vec![];
        let mut count: usize = 0;
        if let TokenVal::ParR = self.stream.look_ahead(1).0.val {
            return (codes, count);
        }
        codes.append(&mut self.load_exp());
        count += 1;
        // if encounter comma, then continue read exp
        if let TokenVal::Comma = self.stream.look_ahead(1).0.val {
            self.assert_next(TokenVal::Comma);
            let (_, c) = self.load_exps();
            count += c;
        }
        (codes, count)
    }

    // auto choose how to perform calulation
    fn auto_op(&mut self, count: &mut usize, op: TokenVal) -> ByteCode {
        let r;
        if *count == 1 {
            r = self.unary_op(op.into());
            *count -= 1;
        } else if *count >= 2 {
            r = self.binary_op(op.into());
            *count -= 1;
        } else {
            panic!("nothing to work!");
        }
        r
    }

    // unary expression
    fn unary_op(&mut self, op: UnaryOP) -> ByteCode {
        ByteCode::UnaryOP(op)
    }

    // binary expression
    fn binary_op(&mut self, op: BinaryOP) -> ByteCode {
        ByteCode::BinaryOP(op)
    }
}
