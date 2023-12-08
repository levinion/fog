mod assert;
mod wrap;
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
                Token::Name(name) => self.call_function(name),
                Token::Let => self.define_local(),
                Token::Eos => break,
                _ => todo!(),
            }
        }
    }

    /// call a function whose name is name
    fn call_function(&mut self, name: String) {
        // load function
        self.load_const(Value::String(name));
        // get the function from global and load to the stack
        self.get_global();
        self.assert_next(Token::ParL);
        // get args
        //TODO: try support args more than one.
        self.load_next_exp();
        self.assert_next(Token::ParR);
        // call function
        self.call(1);
    }

    fn define_local(&mut self) {
        // get variable name
        let name = if let Token::Name(s) = self.lex.next() {
            s
        } else {
            panic!("expected name!")
        };
        self.assert_next(Token::Assign);
        // load value to stack;
        self.load_next_exp();
        self.store_local(name);
    }
}
