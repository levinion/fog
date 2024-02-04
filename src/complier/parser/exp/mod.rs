mod cal;

use crate::core::{
    block::Block,
    bytecode::ByteCode,
    op::{BinaryOP, UnaryOP},
    token::TokenVal,
    value::Value,
};

use super::Parser;

impl Parser {
    pub fn load_exp(&mut self, block: &mut Block) {
        let output = self.handle_infix();
        let mut op_count: usize = 0; // count there is how many values on the stack
        for token in output.into_iter() {
            match token.0.val.clone() {
                TokenVal::Add => self.auto_op(block, &mut op_count, TokenVal::Add),
                TokenVal::Sub => self.auto_op(block, &mut op_count, TokenVal::Sub),
                TokenVal::Mul => self.auto_op(block, &mut op_count, TokenVal::Mul),
                TokenVal::Div => self.auto_op(block, &mut op_count, TokenVal::Div),
                TokenVal::Equal => self.auto_op(block, &mut op_count, TokenVal::Equal),
                TokenVal::NotEq => self.auto_op(block, &mut op_count, TokenVal::NotEq),
                TokenVal::Greater => self.auto_op(block, &mut op_count, TokenVal::Greater),
                TokenVal::Less => self.auto_op(block, &mut op_count, TokenVal::Less),
                TokenVal::GreEq => self.auto_op(block, &mut op_count, TokenVal::GreEq),
                TokenVal::LesEq => self.auto_op(block, &mut op_count, TokenVal::LesEq),
                TokenVal::String(s) => {
                    block.byte_codes.push(ByteCode::LoadValue(Value::String(s)));
                    op_count += 1;
                }
                TokenVal::Name(name) => {
                    block
                        .byte_codes
                        .push(ByteCode::LoadValue(Value::Name(name)));
                    block.byte_codes.push(ByteCode::LoadName);
                    op_count += 1;
                }
                TokenVal::Int(i) => {
                    block.byte_codes.push(ByteCode::LoadValue(Value::Int(i)));
                    op_count += 1;
                }
                TokenVal::Float(f) => {
                    block.byte_codes.push(ByteCode::LoadValue(Value::Float(f)));
                    op_count += 1;
                }
                token => panic!("unexpected token: {:?}", token),
            }
        }
    }

    /// load exps separated by comma
    pub fn load_exps(&mut self, block: &mut Block) -> usize {
        let mut count: usize = 0;
        if let TokenVal::ParR = self.stream.look_ahead(1).0.val {
            return count;
        }
        self.load_exp(block);
        count += 1;
        // if encounter comma, then continue read exp
        if let TokenVal::Comma = self.stream.look_ahead(1).0.val {
            self.assert_next(TokenVal::Comma);
            count += self.load_exps(block);
        }
        count
    }

    // auto choose how to perform calulation
    fn auto_op(&mut self, block: &mut Block, count: &mut usize, op: TokenVal) {
        if *count == 1 {
            self.unary_op(block, op.into());
            *count -= 1;
        } else if *count >= 2 {
            self.binary_op(block, op.into());
            *count -= 1;
        } else {
            panic!("nothing to work!");
        }
    }

    // unary expression
    fn unary_op(&mut self, block: &mut Block, op: UnaryOP) {
        block.byte_codes.push(ByteCode::UnaryOP(op));
    }

    // binary expression
    fn binary_op(&mut self, block: &mut Block, op: BinaryOP) {
        block.byte_codes.push(ByteCode::BinaryOP(op));
    }
}
