mod assert;
mod cal;
pub mod ir;
mod wrapper;

use crate::{
    core::{bytecode::ByteCode, token::Token, value::Value},
    lex::token_stream::TokenStream,
};

use self::ir::IR;

#[derive(Debug)]
pub struct Parser {
    stream: TokenStream,
    pub byte_codes: Vec<ByteCode>,
    pub constants: Vec<Value>,
    pub locals: Vec<String>,
}

impl From<TokenStream> for Parser {
    fn from(value: TokenStream) -> Self {
        let byte_codes = Vec::new();
        let constants = vec![];
        let locals = vec![];
        Self {
            stream: value,
            byte_codes,
            constants,
            locals,
        }
    }
}

impl Parser {
    pub fn into_ir(mut self) -> IR {
        loop {
            if let Some(Token::Eos) = self.parse_once() {
                break;
            }
        }
        IR {
            byte_codes: self.byte_codes,
            constants: self.constants,
            locals: self.locals,
            pc: 0,
        }
    }

    pub fn parse_once(&mut self) -> Option<Token> {
        let token = self.stream.next();
        match token {
            Token::Name(name) => match *self.stream.look_ahead(1) {
                Token::ParL => self.call_function(name),
                Token::Assign => self.assign_local(name),
                _ => {}
            },
            Token::Let => self.define_local(),
            Token::If => self.enter_if(),
            Token::Eos => return Some(Token::Eos),
            Token::CurlyR => return Some(Token::CurlyR),
            _ => panic!("unexpected token: {:?}", token),
        }
        None
    }

    /// call a function with name
    /// eg: println(a, b);
    fn call_function(&mut self, name: String) {
        // load function
        self.load_const(Value::String(name));
        // get the function from global and load to the stack
        self.get_global();
        self.assert_next(Token::ParL);
        // get args
        let argc = self.load_next_exps();
        self.assert_next(Token::ParR);
        self.assert_next(Token::SemiColon);
        // call function
        self.call(argc);
    }

    /// define a local variable
    /// eg: let a = "hello world";
    fn define_local(&mut self) {
        let name = if let Token::Name(s) = self.stream.next() {
            s
        } else {
            panic!("expected name!")
        };
        self.assert_next(Token::Assign);
        self.load_next_exp();
        self.assert_next(Token::SemiColon);
        self.store_local(name);
    }

    /// assign a local variable
    /// eg: a = "hi";
    fn assign_local(&mut self, name: String) {
        self.assert_next(Token::Assign);
        self.load_next_exp();
        self.assert_next(Token::SemiColon);
        self.store_local(name);
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
}
