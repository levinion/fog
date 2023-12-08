mod assert;
mod debug;
mod op;

use std::collections::VecDeque;

use crate::{bytecode::ByteCode, lex::Lex, token::Token, value::Value};

pub struct Parser {
    lex: Lex,
    pub byte_codes: VecDeque<ByteCode>,
    pub constants: Vec<Value>,
    pub locals: Vec<String>,
}

impl From<Lex> for Parser {
    fn from(value: Lex) -> Self {
        let byte_codes = VecDeque::new();
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
    pub fn parse_once(&mut self) -> Option<Token> {
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
            Token::If => self.enter_if(),
            Token::Eos => return Some(Token::Eos),
            Token::CurlyR => return Some(Token::CurlyR),
            _ => todo!(),
        }
        None
    }

    pub fn parse(&mut self) {
        loop {
            if let Some(Token::Eos) = self.parse_once() {
                break;
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

    /// enter if block
    /// eg: ```
    /// if true{
    ///     println("hello")
    /// }```
    fn enter_if(&mut self) {
        self.load_next_exp();
        self.jump_if_false();
        self.parse_block();
    }

    /// parse statement in the block
    fn parse_block(&mut self) {
        self.assert_next(Token::CurlyL);
        self.enter_block();
        loop {
            if let Some(Token::CurlyR) = self.parse_once() {
                break;
            }
        }
        self.leave_block();
    }

    // calulate the result of the expression
    fn calulate_exp(&mut self) {
        todo!()
    }
}
