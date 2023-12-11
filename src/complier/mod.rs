mod lexer;
mod parser;

use walkdir::WalkDir;

use crate::core::{block::Block, ir::IR};

use self::{lexer::Lexer, parser::Parser};
use std::{ffi::OsString, fs::File};

pub struct Complier {}

impl Complier {
    pub fn complie(root: &str) -> IR {
        // walk for all files under root
        let mut blocks = vec![];

        for entry in WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension() == Some(&OsString::from("fog")))
        {
            let path = entry.path();
            dbg!(path);
            let block = complie_file(path.to_str().unwrap());
            blocks.push(block);
        }

        blocks.into()
    }
}

fn complie_file(filename: &str) -> Block {
    let file = File::open(filename).unwrap();
    let stream = Lexer::from(file).into_token_stream();
    Parser::from(stream).parse_file("main".into())
}
