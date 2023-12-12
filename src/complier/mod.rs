mod lexer;
mod parser;

use crate::core::{
    block::{Block, BlockType},
    ir::IR,
};

use self::{lexer::Lexer, parser::Parser};
use std::{fs::File, path::PathBuf};

pub fn complie(root: &str) -> IR {
    // walk for all files under root
    let mut blocks = vec![];

    let module = Block::new(root.into(), BlockType::Module, vec![]);
    let block = handle_module(root, module);
    blocks.push(block);
    blocks.into()
}

fn handle_module(root: &str, mut module: Block) -> Block {
    for entry in std::fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        let name = path.to_str().unwrap();
        let basename = path.file_name().unwrap().to_str().unwrap();
        if path.is_dir() {
            let mut new_module = Block::new(basename.into(), BlockType::Module, vec![]);
            new_module = handle_module(name, new_module);
            module.add_sub_block(new_module);
        } else {
            let block = complie_file(name);
            module.add_sub_block(block);
        }
    }
    module
}

pub fn complie_file(filename: &str) -> Block {
    let file = File::open(filename).unwrap();
    let stream = Lexer::from(file).into_token_stream();
    let path = PathBuf::from(filename);
    let basename = path.file_stem().unwrap().to_str().unwrap();
    Parser::from(stream).parse_file(basename.into())
}
