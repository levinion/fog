mod assert;
mod control_flow;
mod exp;

use std::path::{Path, PathBuf};

use crate::{
    complier::lexer::token_stream::TokenStream,
    core::{
        block::{Block, BlockType},
        bytecode::ByteCode,
        token::TokenVal,
        value::Type,
        value::Value,
    },
};

#[derive(Debug)]
pub struct Parser {
    stream: TokenStream,
    blocks: Vec<Block>,
}

impl From<TokenStream> for Parser {
    fn from(value: TokenStream) -> Self {
        Self {
            stream: value,
            blocks: vec![],
        }
    }
}

impl Parser {
    /// parse all file to a block
    pub fn parse_file(mut self, name: String, path: PathBuf, father: Option<&Block>) -> Vec<Block> {
        let mut block = if let Some(father) = father {
            Block::inherite(father, name, path.clone(), BlockType::File, vec![])
        } else {
            Block::new(name, path.clone(), BlockType::File, vec![])
        };
        loop {
            let token = self.stream.look_ahead(1);
            match &token.0.val {
                TokenVal::Import => {
                    let mut blocks = self.include(&block, &path);
                    self.blocks.append(&mut blocks);
                }
                TokenVal::Name(name) => block.byte_codes.push(self.load_name(name.clone())),
                // let a = 1;
                TokenVal::Let => block.byte_codes.append(&mut self.define_local()),
                // fn main(...)
                TokenVal::Fn => {
                    let block = self.parse_block(&block, path.clone());
                    self.blocks.push(block);
                }
                TokenVal::Eos => break,
                _ => panic!("unexpected token: {:?}", token),
            }
        }
        self.blocks.push(block);
        self.blocks
    }

    pub fn include(&mut self, father: &Block, path: &Path) -> Vec<Block> {
        self.stream.next();
        let name = self.assert_next_name();
        let (possible_path1_exists, r1) = {
            let mut path = path.to_path_buf();
            path.set_extension("");
            let dir_name = path.join(&name);
            let filename = path.join("mod.fog");
            (
                dir_name.is_dir() && filename.is_file(),
                filename.to_string_lossy().to_string(),
            )
        };
        let (possible_path2_exists, r2) = {
            let path = path.parent().unwrap();
            let filename = path.join(name.to_string() + ".fog");
            (filename.exists(), filename.to_string_lossy().to_string())
        };

        let name = {
            if possible_path2_exists && possible_path1_exists {
                panic!("double defined module: {}", name);
            } else if possible_path2_exists {
                r2
            } else if possible_path1_exists {
                r1
            } else {
                panic!("no module named: {}", name);
            }
        };
        crate::complier::complie_file(&name, Some(father)).unwrap()
    }

    /// parse blocks
    // eg: fn test(a, b){...}
    pub fn parse_block(&mut self, father: &Block, path: PathBuf) -> Block {
        let token = self.stream.look_ahead(1);
        match &token.0.val {
            // eg: fn test(a,b){do something here}
            TokenVal::Fn => {
                self.stream.next();
                let name = self.assert_next_name();
                let args = self.parse_fn_args_to_vec();
                let mut block = Block::inherite(father, name, path, BlockType::Fn, args.clone());
                block.args = args;
                block.byte_codes.append(&mut self.parse_bucket());
                block
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
            match token.0.val.clone() {
                TokenVal::Name(name) => {
                    self.stream.next();
                    self.assert_next(TokenVal::Colon);
                    let typ = self.assert_next_type();
                    args.push((name, typ));
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
    fn parse_bucket(&mut self) -> Vec<ByteCode> {
        let mut codes = vec![];
        self.assert_next(TokenVal::CurlyL);
        loop {
            let token = self.stream.look_ahead(1);
            match token.0.val {
                TokenVal::Name(_) => {
                    let token = self.stream.look_ahead(2);
                    match token.0.val {
                        TokenVal::Assign => codes.append(&mut self.assign_local()),
                        TokenVal::ParL => codes.append(&mut self.call_function()),
                        _ => todo!(),
                    }
                }
                TokenVal::Let => codes.append(&mut self.define_local()),
                TokenVal::If => codes.append(&mut self.parse_if()), //self.enter_if(block),
                TokenVal::Fog => codes.append(&mut self.fog_call_function()),
                TokenVal::Eos => panic!("eos!"),
                TokenVal::CurlyR => break,
                TokenVal::Assign => codes.append(&mut self.assign_local()),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
        self.assert_next(TokenVal::CurlyR);
        codes
    }

    fn load_name(&mut self, name: String) -> ByteCode {
        let value = Value::Name(name);
        ByteCode::LoadValue(value)
    }

    /// call normal function with name
    /// eg: print(a, b);
    fn call_function(&mut self) -> Vec<ByteCode> {
        let mut codes = vec![];
        let name = self.assert_next_name();
        codes.push(self.load_name(name));
        self.assert_next(TokenVal::ParL);
        codes.push(ByteCode::LoadName);
        // get args
        let (mut c, argc) = self.load_exps();
        codes.append(&mut c);
        self.assert_next(TokenVal::ParR);
        self.assert_next(TokenVal::SemiColon);
        // call function
        codes.push(ByteCode::CallFunction(argc));
        codes
    }

    fn fog_call_function(&mut self) -> Vec<ByteCode> {
        let mut codes = vec![];
        self.stream.next();
        let name = self.assert_next_name();
        codes.push(self.load_name(name));
        self.assert_next(TokenVal::ParL);
        codes.push(ByteCode::LoadName);
        let (mut c, argc) = self.load_exps();
        codes.append(&mut c);
        self.assert_next(TokenVal::ParR);
        self.assert_next(TokenVal::SemiColon);
        codes.push(ByteCode::FogCallFunction(argc));
        codes
    }

    // eg: value.method(exps);
    // fn call_method(&mut self, block: &mut Block) {
    //     let name = self.assert_next_name();
    //     self.load_name(block, name);
    //     block.byte_codes.push(ByteCode::LoadName);
    //     self.assert_next(TokenVal::Dot);
    //     // get method name
    //     let name = self.assert_next_name();
    //     block
    //         .byte_codes
    //         .push(ByteCode::LoadValue(Value::String(name)));
    //     self.assert_next(TokenVal::ParL);
    //     // get args
    //     let argc = self.load_exps(block);
    //     self.assert_next(TokenVal::ParR);
    //     self.assert_next(TokenVal::SemiColon);
    //     block.byte_codes.push(ByteCode::CallMethod(argc));
    // }

    /// define a local variable
    /// eg: let a = "hello world";
    fn define_local(&mut self) -> Vec<ByteCode> {
        let mut codes = vec![];
        self.assert_next(TokenVal::Let);
        let name = self.assert_next_name();
        codes.push(ByteCode::LoadValue(Value::Name(name)));
        self.assert_next(TokenVal::Assign);
        codes.append(&mut self.load_exp());
        self.assert_next(TokenVal::SemiColon);
        codes.push(ByteCode::StoreLocal);
        codes
    }

    /// assign a local variable
    /// eg: a = "hi";
    fn assign_local(&mut self) -> Vec<ByteCode> {
        let mut codes = vec![];
        let name = self.assert_next_name();
        codes.push(self.load_name(name));
        self.assert_next(TokenVal::Assign);
        codes.append(&mut self.load_exp());
        codes.push(ByteCode::StoreLocal);
        self.assert_next(TokenVal::SemiColon);
        codes
    }
}
