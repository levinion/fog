mod assert;
mod control_flow;
mod exp;
mod wrapper;

use crate::{
    complier::lexer::token_stream::TokenStream,
    core::{
        block::{Block, BlockType},
        ir::IR,
        token::Token,
        value::Value,
    },
};

#[derive(Debug)]
pub struct Parser {
    stream: TokenStream,
    block: Block, // file block
}

impl From<TokenStream> for Parser {
    fn from(value: TokenStream) -> Self {
        let mut block = Block::new(BlockType::File);
        // TODO: Get Filename
        block.name = "filename".into();
        Self {
            stream: value,
            block,
        }
    }
}

impl Parser {
    pub fn into_ir(mut self) -> IR {
        self.parse_blocks().into()
    }

    /// parse the file to blocks
    pub fn parse_blocks(&mut self) -> Vec<Block> {
        let mut blocks = vec![];
        loop {
            let token = self.stream.next();
            match token {
                Token::Fn => {
                    let mut block = Block::inherite(&self.block, BlockType::Fn);
                    let name = if let Token::Name(name) = self.stream.next() {
                        name
                    } else {
                        panic!("expected name!");
                    };
                    block.name = name;
                    self.assert_next(Token::ParL);
                    self.assert_next(Token::ParR);
                    self.parse_curly_pair(&mut block);
                    blocks.push(block);
                }
                Token::Eos => break,
                _ => panic!("invalid block"),
            }
        }
        blocks
    }

    // parse a block until meet CurlyR
    fn parse_curly_pair(&mut self, block: &mut Block) {
        self.assert_next(Token::CurlyL);
        loop {
            let token = self.stream.next();
            match token {
                Token::Name(name) => match *self.stream.look_ahead(1) {
                    Token::ParL => self.call_function(block, name),
                    Token::Assign => self.assign_local(block, name),
                    _ => panic!("not supported now!"),
                },
                Token::Let => self.define_local(block),
                Token::At => self.call_super_function(block),
                Token::If => self.enter_if(block),
                Token::Fog => self.call_fog_function(block),
                Token::Eos => panic!("eos!"),
                Token::CurlyR => break,
                _ => panic!("unexpected token: {:?}", token),
            }
        }
    }

    /// call super function with name
    /// eg: @print(a, b);
    fn call_super_function(&mut self, block: &mut Block) {
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

    fn call_fog_function(&mut self, block: &mut Block) {
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
