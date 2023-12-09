use crate::core::{
    bytecode::ByteCode,
    op::{BinaryOP, UnaryOP},
    token::Token,
    value::Value,
};

use super::Parser;

impl Parser {
    pub fn load_next_exp(&mut self) {
        let output = self.handle_infix();
        let mut op_count: usize = 0; // count there is how many values on the stack
        for token in output.iter() {
            match token.clone() {
                Token::Add => self.auto_op(&mut op_count, Token::Add),
                Token::Sub => self.auto_op(&mut op_count, Token::Sub),
                Token::Mul => self.auto_op(&mut op_count, Token::Mul),
                Token::Div => self.auto_op(&mut op_count, Token::Div),
                Token::String(s) => {
                    self.load_const(Value::String(s));
                    op_count += 1;
                }
                Token::Name(name) => {
                    self.load_local(name);
                    op_count += 1;
                }
                Token::Int(i) => {
                    self.load_const(Value::Int(i));
                    op_count += 1;
                }
                Token::Float(f) => {
                    self.load_const(Value::Float(f));
                    op_count += 1;
                }
                token => panic!("unexpected token: {:?}", token),
            }
        }
    }

    /// load consts separated by comma
    pub fn load_next_exps(&mut self) -> usize {
        self.load_next_exp();
        let mut count: usize = 1;
        // if encounter comma, then continue read exp
        if let &Token::Comma = self.stream.look_ahead(1) {
            self.assert_next(Token::Comma);
            count += self.load_next_exps();
        }
        count
    }

    /// load a value to stack
    pub fn load_const(&mut self, value: Value) {
        // add argument name to constants table and then load it to stack

        // if there exists the value, then just return it.
        if let Some(index) = self.constants.iter().position(|x| x == &value) {
            self.byte_codes.push(ByteCode::LoadConst { index });
            return;
        }
        self.constants.push(value);
        self.byte_codes.push(ByteCode::LoadConst {
            index: self.constants.len() - 1,
        });
    }

    // get from global and load to the stack
    pub fn get_global(&mut self) {
        self.byte_codes.push(ByteCode::GetGlobal);
    }

    // take the function and args then call it.
    pub fn call(&mut self, argc: usize) {
        self.byte_codes.push(ByteCode::CallFunction { argc });
    }

    // store the name to locals and return its index
    pub fn store_local(&mut self, name: String) {
        // if there exists the value, then just return it.
        if let Some(index) = self.locals.iter().position(|x| x == &name) {
            self.byte_codes.push(ByteCode::StoreLocal { index });
            return;
        }

        self.locals.push(name);
        self.byte_codes.push(ByteCode::StoreLocal {
            index: self.locals.len() - 1,
        });
    }

    // load local variable from the locals
    pub fn load_local(&mut self, name: String) {
        let index = self
            .locals
            .iter()
            .rposition(|x| *x == name)
            .unwrap_or_else(|| panic!("name not found: {name}"));
        self.byte_codes.push(ByteCode::LoadLocal { index });
    }

    pub fn jump_if_false(&mut self) {
        self.byte_codes.push(ByteCode::JumpIfFalse);
    }

    pub fn enter_block(&mut self) {
        self.byte_codes.push(ByteCode::EnterBlock);
    }

    pub fn leave_block(&mut self) {
        self.byte_codes.push(ByteCode::LeaveBlock);
    }

    pub fn auto_op(&mut self, count: &mut usize, op: Token) {
        if *count == 1 {
            self.unary_op(op.into());
            *count -= 1;
        } else if *count >= 2 {
            self.binary_op(op.into());
            *count -= 1;
        } else {
            panic!("nothing to work!");
        }
    }

    fn unary_op(&mut self, op: UnaryOP) {
        self.byte_codes.push(ByteCode::UnaryOP(op));
    }

    fn binary_op(&mut self, op: BinaryOP) {
        self.byte_codes.push(ByteCode::BinaryOP(op));
    }
}
