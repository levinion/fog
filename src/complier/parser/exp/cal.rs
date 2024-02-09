use std::collections::VecDeque;

use crate::core::{
    op::InfixBinaryOP,
    token::{Token, TokenVal},
};

use super::Parser;

impl Parser {
    // TODO: FIX Function Nested
    pub fn handle_infix(&mut self) -> Vec<Token> {
        let mut output_stack: Vec<Token> = Vec::new();
        let mut op_stack: VecDeque<Token> = VecDeque::new();
        let mut par_count = 0; // count par pairs
        loop {
            let token = self.stream.look_ahead(1);
            match token.0.val {
                TokenVal::Int(_) | TokenVal::Float(_) | TokenVal::String(_) | TokenVal::Type(_) => {
                    output_stack.push(self.stream.next());
                }
                TokenVal::Name(_) => {
                    output_stack.push(self.stream.next());
                    let token = self.stream.look_ahead(1);
                    if let TokenVal::ParL = token.0.val {
                        self.stream.next(); // ParL
                        let mut tokens = self.handle_infix();
                        output_stack.append(&mut tokens);
                        let token = self.stream.next(); // ParR
                        output_stack.push(token);
                    }
                }
                TokenVal::Add
                | TokenVal::Sub
                | TokenVal::Mul
                | TokenVal::Div
                | TokenVal::Equal
                | TokenVal::NotEq
                | TokenVal::Greater
                | TokenVal::Less
                | TokenVal::GreEq
                | TokenVal::LesEq => {
                    let token = self.stream.next();
                    self.handle_op(&mut op_stack, &mut output_stack, token);
                }
                TokenVal::ParL => {
                    par_count += 1;
                    let token = self.stream.next();
                    op_stack.push_back(token);
                }
                TokenVal::ParR => {
                    par_count -= 1;
                    // exit condition: when the second ParR appears
                    if par_count < 0 {
                        clean_loop(&mut op_stack, &mut output_stack);
                        break;
                    }

                    loop {
                        let top = op_stack.pop_back().unwrap();
                        match top.0.infix_binary_op() {
                            InfixBinaryOP::ParL => break,
                            _ => output_stack.push(top),
                        }
                    }
                }
                TokenVal::SemiColon | TokenVal::CurlyL | TokenVal::Comma => {
                    // exit condition: when some token appears
                    clean_loop(&mut op_stack, &mut output_stack);
                    break;
                }
                _ => panic!("unexpected token!: {:?}", token),
            }
        }
        output_stack
    }

    fn handle_op(
        &mut self,
        op_stack: &mut VecDeque<Token>,
        output_stack: &mut Vec<Token>,
        token: Token,
    ) {
        loop {
            let top = op_stack.back();
            match top {
                Some(top) => {
                    if token.0.infix_binary_op().priority() > top.0.infix_binary_op().priority() {
                        op_stack.push_back(token);
                        break;
                    }
                    let top = op_stack.pop_back().unwrap();
                    output_stack.push(top);
                }
                None => {
                    op_stack.push_back(token);
                    break;
                }
            }
        }
    }
}

fn clean_loop(op_stack: &mut VecDeque<Token>, output_stack: &mut Vec<Token>) {
    loop {
        let top = op_stack.pop_back();
        match top {
            Some(top) => output_stack.push(top),
            None => break,
        }
    }
}
