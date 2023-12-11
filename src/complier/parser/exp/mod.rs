mod cal;

use crate::core::{
    block::Block,
    bytecode::ByteCode,
    op::{BinaryOP, UnaryOP},
    token::Token,
    value::Value,
};

use super::Parser;

impl Parser {
    pub fn load_exp(&mut self, block: &mut Block) {
        let output = self.handle_infix();
        let mut op_count: usize = 0; // count there is how many values on the stack
        for token in output.into_iter() {
            match token {
                Token::Add => self.auto_op(block, &mut op_count, Token::Add),
                Token::Sub => self.auto_op(block, &mut op_count, Token::Sub),
                Token::Mul => self.auto_op(block, &mut op_count, Token::Mul),
                Token::Div => self.auto_op(block, &mut op_count, Token::Div),
                Token::Equal => self.auto_op(block, &mut op_count, Token::Equal),
                Token::NotEq => self.auto_op(block, &mut op_count, Token::NotEq),
                Token::Greater => self.auto_op(block, &mut op_count, Token::Greater),
                Token::Less => self.auto_op(block, &mut op_count, Token::Less),
                Token::GreEq => self.auto_op(block, &mut op_count, Token::GreEq),
                Token::LesEq => self.auto_op(block, &mut op_count, Token::LesEq),
                Token::String(s) => {
                    self.load_const(block, Value::String(s));
                    op_count += 1;
                }
                Token::Name(name) => {
                    self.load_local(block, name);
                    op_count += 1;
                }
                Token::Int(i) => {
                    self.load_const(block, Value::Int(i));
                    op_count += 1;
                }
                Token::Float(f) => {
                    self.load_const(block, Value::Float(f));
                    op_count += 1;
                }
                token => panic!("unexpected token: {:?}", token),
            }
        }
    }

    /// load exps separated by comma
    pub fn load_exps(&mut self, block: &mut Block) -> usize {
        let mut count: usize = 0;
        if let &Token::ParR = self.stream.look_ahead(1) {
            return count;
        }
        self.load_exp(block);
        count += 1;
        // if encounter comma, then continue read exp
        if let &Token::Comma = self.stream.look_ahead(1) {
            self.assert_next(Token::Comma);
            count += self.load_exps(block);
        }
        count
    }

    // auto choose how to perform calulation
    fn auto_op(&mut self, block: &mut Block, count: &mut usize, op: Token) {
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
