mod assert;
mod control_flow;
mod exp;
mod wrapper;

use crate::{
    complier::lexer::token_stream::TokenStream,
    core::{
        block::{Block, BlockType},
        bytecode::{Decorate, FunctionType},
        token::TokenVal,
        value::Type,
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
            match &token.val {
                TokenVal::Name(name) => self.load_name(&mut block, name.clone()),
                // let a = 1;
                TokenVal::Let => self.define_local(&mut block),
                // fn main(...)
                TokenVal::Fn => self.parse_blocks(&mut block),
                TokenVal::Eos => break,
                _ => panic!("unexpected token: {:?}", token),
            }
        }
        block
    }

    /// parse blocks
    // eg: fn test(a, b){...}
    pub fn parse_blocks(&mut self, father: &mut Block) {
        let token = self.stream.look_ahead(1);
        match &token.val {
            // eg: fn test(a,b){do something here}
            TokenVal::Fn => {
                self.stream.next();
                let (name, name_t) = self.assert_next_name();
                let args = self.parse_fn_args_to_vec();
                let mut block = Block::inherite(father, name, BlockType::Fn, args.clone());
                block.args = args;
                self.parse_curly_pair(&mut block);
                father.add_sub_block(block);
            }
            token => panic!("invalid block! found token: {token:?}"),
        }
    }

    /// This function is used by parse_blocks.
    /// It should not be used in other position.
    /// eg: fn test(a:type,b:type){...} -> get \[("a",type),("b",type)\]
    fn parse_fn_args_to_vec(&mut self) -> Vec<(String, Type)> {
        let mut args = vec![];
        self.assert_next(TokenVal::ParL);
        loop {
            let token = self.stream.look_ahead(1);
            match token.clone().val {
                TokenVal::Name(name) => {
                    self.stream.next();
                    self.assert_next(TokenVal::Colon);
                    let (typ, typ_t) = self.assert_next_name();
                    args.push((name, typ.into()));
                }
                TokenVal::ParR => break,
                TokenVal::Comma => self.assert_next(TokenVal::Comma),
                _ => panic!("invalid token!"),
            }
        }
        self.assert_next(TokenVal::ParR);
        args
    }

    /// This function is used by parse_blocks.
    /// It should not be used in other position.
    /// Parse a block until meet CurlyR.
    /// This is the true function that handle the logic.
    /// eg: {...}
    fn parse_curly_pair(&mut self, block: &mut Block) {
        self.assert_next(TokenVal::CurlyL);
        loop {
            let token = self.stream.look_ahead(1);
            match token.val {
                TokenVal::Name(_) => {
                    let token = self.stream.look_ahead(2);
                    match token.val {
                        TokenVal::Assign => self.assign_local(block),
                        TokenVal::ParL => self.call_function(block),
                        _ => todo!(),
                    }
                }
                TokenVal::Let => self.define_local(block),
                TokenVal::If => self.enter_if(block),
                TokenVal::Fog => self.handle_fog(block),
                TokenVal::Eos => panic!("eos!"),
                TokenVal::CurlyR => break,
                TokenVal::Assign => self.assign_local(block),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
        self.assert_next(TokenVal::CurlyR);
    }

    fn load_name(&mut self, block: &mut Block, name: String) {
        let value = Value::Name(name);
        wrapper::load_value(block, value);
    }

    fn handle_fog(&mut self, block: &mut Block) {
        self.stream.next();
        wrapper::decorate(block, Decorate::Fog);
    }

    /// call normal function with name
    /// eg: print(a, b);
    fn call_function(&mut self, block: &mut Block) {
        let (name, name_t) = self.assert_next_name();
        self.load_name(block, name);
        self.assert_next(TokenVal::ParL);
        wrapper::load_name(block);
        // get args
        let argc = self.load_exps(block);
        self.assert_next(TokenVal::ParR);
        self.assert_next(TokenVal::SemiColon);
        // call function
        wrapper::call_function(block, argc, FunctionType::Undefined);
    }

    // eg: value.method(exps);
    fn call_method(&mut self, block: &mut Block) {
        let (name, name_t) = self.assert_next_name();
        self.load_name(block, name);
        wrapper::load_name(block);
        self.assert_next(TokenVal::Dot);
        // get method name
        let (name, name_t) = self.assert_next_name();
        wrapper::load_value(block, Value::String(name));
        self.assert_next(TokenVal::ParL);
        // get args
        let argc = self.load_exps(block);
        self.assert_next(TokenVal::ParR);
        self.assert_next(TokenVal::SemiColon);
        wrapper::call_method(block, argc);
    }

    /// define a local variable
    /// eg: let a = "hello world";
    fn define_local(&mut self, block: &mut Block) {
        self.assert_next(TokenVal::Let);
        let (name, name_t) = self.assert_next_name();
        self.assert_next(TokenVal::Assign);
        wrapper::load_value(block, Value::Name(name));
        self.load_exp(block);
        self.assert_next(TokenVal::SemiColon);
        wrapper::store_local(block);
    }

    /// assign a local variable
    /// eg: a = "hi";
    fn assign_local(&mut self, block: &mut Block) {
        let (name, name_t) = self.assert_next_name();
        self.load_name(block, name);
        self.assert_next(TokenVal::Assign);
        self.load_exp(block);
        self.assert_next(TokenVal::SemiColon);
        wrapper::store_local(block);
    }
}
