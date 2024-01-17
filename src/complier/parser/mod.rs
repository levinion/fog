mod assert;
mod control_flow;
mod exp;
mod wrapper;

use crate::{
    complier::lexer::token_stream::TokenStream,
    core::{
        block::{Block, BlockType},
        bytecode::{Decorate, FunctionType},
        token::Token,
        value::Value,
    },
};

#[derive(Debug)]
pub struct Parser {
    stream: TokenStream,
}

impl From<TokenStream> for Parser {
    fn from(value: TokenStream) -> Self {
        Self { stream: value }
    }
}

impl Parser {
    /// parse all file to a block
    pub fn parse_file(&mut self, name: String, father: Option<&Block>) -> Block {
        let mut block = if let Some(father) = father {
            Block::inherite(father, name, BlockType::File, vec![])
        } else {
            Block::new(name, BlockType::File, vec![])
        };
        loop {
            let token = self.stream.look_ahead(1);
            match token {
                Token::Name(name) => self.handle_name(&mut block, name.clone()),
                // let a = 1;
                Token::Let => self.define_local(&mut block),
                // fn main(...)
                Token::Fn => self.parse_blocks(&mut block),
                Token::Eos => break,
                _ => panic!("unexpected token: {:?}", token),
            }
        }
        block
    }

    /// parse blocks
    // eg: fn test(a, b){...}
    pub fn parse_blocks(&mut self, father: &mut Block) {
        let token = self.stream.look_ahead(1);
        match token {
            // eg: fn test(a,b){do something here}
            Token::Fn => {
                self.stream.next();
                let name = if let Token::Name(name) = self.stream.next() {
                    name
                } else {
                    panic!("expected name!");
                };
                let mut args = self.parse_fn_args_to_vec();
                let mut block = Block::inherite(father, name, BlockType::Fn, args.clone());
                block.locals.append(&mut args);
                self.parse_curly_pair(&mut block);
                father.add_sub_block(block);
            }
            token => panic!("invalid block! found token: {token:?}"),
        }
    }

    /// This function is used by parse_blocks.
    /// It should not be used in other position.
    /// eg: fn test(a,b){...} -> get \["a","b"\]
    fn parse_fn_args_to_vec(&mut self) -> Vec<String> {
        let mut args = vec![];
        self.assert_next(Token::ParL);
        loop {
            let token = self.stream.look_ahead(1);
            match token {
                Token::Name(name) => {
                    args.push(name.clone());
                    self.stream.next();
                }
                Token::ParR => break,
                Token::Comma => self.assert_next(Token::Comma),
                _ => panic!("invalid token!"),
            }
        }
        self.assert_next(Token::ParR);
        args
    }

    /// This function is used by parse_blocks.
    /// It should not be used in other position.
    /// Parse a block until meet CurlyR.
    /// This is the true function that handle the logic.
    /// eg: {...}
    fn parse_curly_pair(&mut self, block: &mut Block) {
        self.assert_next(Token::CurlyL);
        loop {
            let token = self.stream.look_ahead(1);
            match token {
                Token::Name(name) => self.handle_name(block, name.clone()),
                Token::Let => self.define_local(block),
                Token::ParL => self.call_function(block),
                Token::If => self.enter_if(block),
                Token::Fog => self.handle_fog(block),
                Token::Eos => panic!("eos!"),
                Token::CurlyR => break,
                _ => panic!("unexpected token: {:?}", token),
            }
        }
        self.assert_next(Token::CurlyR);
    }

    // when meets the name, then load it to the stack. Then we can handle it later.
    fn handle_name(&mut self, block: &mut Block, name: String) {
        self.stream.next();
        let value = Value::String(name);
        // add argument name to constants table and then load it to stack
        wrapper::load_const(block, value);
    }

    fn handle_fog(&mut self, block: &mut Block) {
        self.stream.next();
        wrapper::decorate(block, Decorate::Fog);
    }

    /// call normal function with name
    /// eg: print(a, b);
    fn call_function(&mut self, block: &mut Block) {
        self.assert_next(Token::ParL);
        // get args
        let argc = self.load_exps(block);
        self.assert_next(Token::ParR);
        self.assert_next(Token::SemiColon);
        // call function
        wrapper::call_function(block, argc, FunctionType::Undefined);
    }

    // eg: value.method(exps);
    fn call_method(&mut self, block: &mut Block, name: String) {
        // get value
        wrapper::load_local(block, name);
        self.assert_next(Token::Dot);
        // get method name
        let name = if let Token::Name(name) = self.stream.next() {
            name
        } else {
            panic!("expected some function name!");
        };
        wrapper::load_const(block, Value::String(name));
        self.assert_next(Token::ParL);
        // get args
        let argc = self.load_exps(block);
        self.assert_next(Token::ParR);
        self.assert_next(Token::SemiColon);
        wrapper::call_method(block, argc);
    }

    /// define a local variable
    /// eg: let a = "hello world";
    fn define_local(&mut self, block: &mut Block) {
        self.assert_next(Token::Let);
        let name = if let Token::Name(s) = self.stream.next() {
            s
        } else {
            panic!("expected name!")
        };
        self.assert_next(Token::Assign);
        self.load_exp(block);
        self.assert_next(Token::SemiColon);
        wrapper::store_local(block, name);
    }

    /// assign a local variable
    /// eg: a = "hi";
    fn assign_local(&mut self, block: &mut Block, name: String) {
        self.assert_next(Token::Assign);
        self.load_exp(block);
        self.assert_next(Token::SemiColon);
        wrapper::store_local(block, name);
    }
}
