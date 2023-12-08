use crate::{bytecode::ByteCode, token::Token, value::Value};

use super::Parser;

impl Parser {
    /// load next value (constant or variable) to stack
    pub fn load_next_exp(&mut self) {
        match self.lex.next() {
            Token::String(s) => {
                self.load_const(Value::String(s));
            }
            Token::Name(name) => {
                self.load_local(name);
            }
            _ => todo!(),
        }
    }

    /// load consts separated by comma
    pub fn load_next_exps(&mut self) {
        self.load_next_exp();
        if let Token::Comma = self.lex.next() {
            self.load_next_exps()
        } else {
            //TODO: try fix it.

            // self.lex.redraw();
            todo!()
        }
    }

    /// load a value to stack
    pub fn load_const(&mut self, value: Value) {
        // add argument name to constants table and then load it to stack
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
}
