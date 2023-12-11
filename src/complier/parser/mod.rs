mod assert;
mod control_flow;
mod exp;
mod wrapper;

use crate::{
    complier::lexer::token_stream::TokenStream,
    core::{
        block::{Block, BlockType},
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
    pub fn parse_file(&mut self, name: String) -> Block {
        // TODO: Get Filename
        let mut block = Block::new(name, BlockType::File, vec![]);
        loop {
            let token = self.stream.look_ahead(1);
            match token {
                Token::Name(name) => self.handle_name(&mut block, name.clone()),
                // let a = 1;
                Token::Let => self.define_local(&mut block),
                // @println(...);
                Token::At => self.call_super_function(&mut block),
                // if a > 0 {...}
                Token::If => self.enter_if(&mut block),
                // fog call(...);
                Token::Fog => self.call_fog_function(&mut block),
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
                let args = self.parse_fn_args_to_vec();
                let mut block = Block::inherite(father, name, BlockType::Fn, args);
                self.parse_curly_pair(&mut block);
                father.add_sub_block(block);
            }
            token => panic!("invalid block! found token: {token:?}"),
        }
    }

    // eg: fn test(a,b){} -> get ["a","b"]
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

    // parse a block until meet CurlyR
    fn parse_curly_pair(&mut self, block: &mut Block) {
        self.assert_next(Token::CurlyL);
        loop {
            let token = self.stream.look_ahead(1);
            match token {
                Token::Name(name) => self.handle_name(block, name.clone()),
                Token::Let => self.define_local(block),
                Token::At => self.call_super_function(block),
                Token::If => self.enter_if(block),
                Token::Fog => self.call_fog_function(block),
                Token::Eos => panic!("eos!"),
                Token::CurlyR => break,
                _ => panic!("unexpected token: {:?}", token),
            }
        }
        self.assert_next(Token::CurlyR);
    }

    fn handle_name(&mut self, block: &mut Block, name: String) {
        let name = name.clone();
        self.stream.next();
        match *self.stream.look_ahead(1) {
            Token::ParL => self.call_function(block, name),
            Token::Assign => self.assign_local(block, name),
            _ => panic!("not supported now!"),
        }
    }

    /// call super function with name
    /// eg: @print(a, b);
    fn call_super_function(&mut self, block: &mut Block) {
        self.assert_next(Token::At);
        let name = if let Token::Name(name) = self.stream.next() {
            name
        } else {
            panic!("expected some super function name!");
        };
        // load function
        self.load_const(block, Value::String(name));
        // get the function from global and load to the stack
        self.get_global(block);
        self.assert_next(Token::ParL);
        // get args
        let argc = self.load_exps(block);
        self.assert_next(Token::ParR);
        self.assert_next(Token::SemiColon);
        // call function
        self.call_super(block, argc);
    }

    /// call normal function with name
    /// eg: print(a, b);
    fn call_function(&mut self, block: &mut Block, name: String) {
        // load function
        self.load_const(block, Value::String(name));
        self.assert_next(Token::ParL);
        // get args
        let argc = self.load_exps(block);
        self.assert_next(Token::ParR);
        self.assert_next(Token::SemiColon);
        // call function
        self.call(block, argc);
    }

    // eg: fog test(a);
    fn call_fog_function(&mut self, block: &mut Block) {
        self.assert_next(Token::Fog);
        let name = if let Token::Name(name) = self.stream.next() {
            name
        } else {
            panic!("expected some function name!");
        };
        self.load_const(block, Value::String(name));
        self.assert_next(Token::ParL);
        // get args
        let argc = self.load_exps(block);
        self.assert_next(Token::ParR);
        self.assert_next(Token::SemiColon);
        // call function
        self.call_fog(block, argc);
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
        self.store_local(block, name);
    }

    /// assign a local variable
    /// eg: a = "hi";
    fn assign_local(&mut self, block: &mut Block, name: String) {
        self.assert_next(Token::Assign);
        self.load_exp(block);
        self.assert_next(Token::SemiColon);
        self.store_local(block, name);
    }
}
