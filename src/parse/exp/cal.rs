use std::collections::VecDeque;

use crate::core::{op::InfixBinaryOP, token::Token};

use super::Parser;

impl Parser {
    pub fn handle_infix(&mut self) -> Vec<Token> {
        let mut output_stack: Vec<Token> = Vec::new();
        let mut op_stack: VecDeque<InfixBinaryOP> = VecDeque::new();
        let mut par_count = 0; // count par pairs
        loop {
            let token = self.stream.look_ahead(1);
            match *token {
                Token::Int(_) | Token::Float(_) | Token::String(_) | Token::Name(_) => {
                    output_stack.push(self.stream.next());
                }
                Token::Add => {
                    let op = InfixBinaryOP::Add;
                    self.stream.next();
                    self.handle_op(&mut op_stack, &mut output_stack, op);
                }
                Token::Sub => {
                    let op = InfixBinaryOP::Sub;
                    self.stream.next();
                    self.handle_op(&mut op_stack, &mut output_stack, op);
                }
                Token::Mul => {
                    let op = InfixBinaryOP::Mul;
                    self.stream.next();
                    self.handle_op(&mut op_stack, &mut output_stack, op);
                }
                Token::Div => {
                    let op = InfixBinaryOP::Div;
                    self.stream.next();
                    self.handle_op(&mut op_stack, &mut output_stack, op);
                }
                Token::Equal => {
                    let op = InfixBinaryOP::Equal;
                    self.stream.next();
                    self.handle_op(&mut op_stack, &mut output_stack, op);
                }
                Token::NotEq => {
                    let op = InfixBinaryOP::NotEq;
                    self.stream.next();
                    self.handle_op(&mut op_stack, &mut output_stack, op);
                }
                Token::Greater => {
                    let op = InfixBinaryOP::Greater;
                    self.stream.next();
                    self.handle_op(&mut op_stack, &mut output_stack, op);
                }
                Token::Less => {
                    let op = InfixBinaryOP::Less;
                    self.stream.next();
                    self.handle_op(&mut op_stack, &mut output_stack, op);
                }
                Token::GreEq => {
                    let op = InfixBinaryOP::GreEq;
                    self.stream.next();
                    self.handle_op(&mut op_stack, &mut output_stack, op);
                }
                Token::LesEq => {
                    let op = InfixBinaryOP::LesEq;
                    self.stream.next();
                    self.handle_op(&mut op_stack, &mut output_stack, op);
                }
                Token::ParL => {
                    par_count += 1;
                    let op = InfixBinaryOP::ParL;
                    self.stream.next();
                    op_stack.push_back(op);
                }
                Token::ParR => {
                    par_count -= 1;
                    // exit condition: when the second ParR appears
                    if par_count < 0 {
                        clean_loop(&mut op_stack, &mut output_stack);
                        break;
                    }
                    self.stream.next();

                    loop {
                        let top = op_stack.pop_back().unwrap();
                        match top {
                            InfixBinaryOP::ParL => break,
                            _ => output_stack.push(top.into()),
                        }
                    }
                }
                Token::SemiColon | Token::CurlyL => {
                    // exit condition: when some token appears
                    clean_loop(&mut op_stack, &mut output_stack);
                    break;
                }
                _ => panic!("unexpected token!"),
            }
        }
        output_stack
    }

    fn handle_op(
        &mut self,
        op_stack: &mut VecDeque<InfixBinaryOP>,
        output_stack: &mut Vec<Token>,
        op: InfixBinaryOP,
    ) {
        loop {
            let top = op_stack.back();
            match top {
                Some(top) => {
                    if op.priority() > top.priority() {
                        op_stack.push_back(op);
                        break;
                    }
                    let top = op_stack.pop_back().unwrap();
                    output_stack.push(top.into());
                }
                None => {
                    op_stack.push_back(op);
                    break;
                }
            }
        }
    }
}

fn clean_loop(op_stack: &mut VecDeque<InfixBinaryOP>, output_stack: &mut Vec<Token>) {
    loop {
        let top = op_stack.pop_back();
        match top {
            Some(top) => output_stack.push(top.into()),
            None => break,
        }
    }
}
