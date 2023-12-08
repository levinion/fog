use crate::core::{
    bytecode::ByteCode,
    op::{BinaryOP, UnaryOP},
    token::Token,
    value::Value,
};

use super::Parser;

impl Parser {
    pub fn load_next_exp(&mut self) {
        self.load_next_token();
        loop {
            match *self.stream.look_ahead(1) {
                Token::Add => self.binary_op(BinaryOP::Add),
                Token::Sub => self.binary_op(BinaryOP::Sub),
                Token::Mul => self.binary_op(BinaryOP::Mul),
                Token::Div => self.binary_op(BinaryOP::Div),
                _ => break,
            }
        }
    }

    /// load next value (constant or variable) to stack
    pub fn load_next_token(&mut self) {
        match self.stream.next() {
            Token::String(s) => self.load_const(Value::String(s)),
            Token::Name(name) => self.load_local(name),
            Token::Bool(b) => self.load_const(Value::Bool(b)),
            Token::Int(i) => self.load_const(Value::Int(i)),
            Token::Float(f) => self.load_const(Value::Float(f)),
            Token::Sub => self.unary_op(UnaryOP::Sub),
            _ => todo!(),
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
        let index = self.locals.iter().rposition(|x| *x == name).unwrap();
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

    pub fn unary_op(&mut self, op: UnaryOP) {
        self.load_next_token();
        self.byte_codes.push(ByteCode::UnaryOP(op));
    }

    pub fn binary_op(&mut self, op: BinaryOP) {
        self.stream.next(); // op
        self.load_next_token(); // second op number.
        self.byte_codes.push(ByteCode::BinaryOP(op));
    }
}
