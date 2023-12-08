mod assert;
mod debug;
mod op;

use crate::{bytecode::ByteCode, lex::Lex, token::Token, value::Value};

pub struct Parser {
    lex: Lex,
    pub byte_codes: Vec<ByteCode>,
    pub constants: Vec<Value>,
    pub locals: Vec<String>,
}

impl From<Lex> for Parser {
    fn from(value: Lex) -> Self {
        let byte_codes = vec![];
        let constants = vec![];
        let locals = vec![];
        Self {
            lex: value,
            byte_codes,
            constants,
            locals,
        }
    }
}

impl Parser {
    pub fn parse(&mut self) {
        loop {
            let token = self.lex.next();
            match token {
                Token::Name(name) => {
                    if self.lex.look_ahead(1) == &Token::ParL {
                        self.call_function(name);
                    } else {
                        self.assign_local(name);
                    }
                }
                Token::Let => self.define_local(),
                Token::Eos => break,
                _ => todo!(),
            }
        }
    }

    /// call a function with name
    /// eg: println(a, b)
    fn call_function(&mut self, name: String) {
        // load function
        self.load_const(Value::String(name));
        // get the function from global and load to the stack
        self.get_global();
        self.assert_next(Token::ParL);
        // get args
        let argc = self.load_next_exps();
        self.assert_next(Token::ParR);
        // call function
        self.call(argc);
    }

    /// define a local variable
    /// eg: let a = "hello world"
    fn define_local(&mut self) {
        let name = if let Token::Name(s) = self.lex.next() {
            s
        } else {
            panic!("expected name!")
        };
        self.assert_next(Token::Assign);
        self.load_next_exp();
        self.store_local(name);
    }

    /// assign a local variable
    /// eg: a = "hi"
    fn assign_local(&mut self, name: String) {
        self.assert_next(Token::Assign);
        self.load_next_exp();
        self.store_local(name)
    }
}
