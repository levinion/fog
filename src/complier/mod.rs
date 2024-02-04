mod lexer;
pub mod optimizer;
mod parser;

use anyhow::{Context, Result};

use crate::core::{block::Block, ir::IR1};

use self::{lexer::Lexer, parser::Parser};
use std::{fs::File, io::Cursor, path::PathBuf};

pub fn complie_project(root: &str) -> Result<IR1> {
    let blocks = complie_file(root, None)?;
    Ok(IR1(blocks))
}

pub fn complie_file(filename: &str, father: Option<&Block>) -> Result<Vec<Block>> {
    let file = File::open(filename)?;
    let stream = Lexer::from(file).into_token_stream();
    let path = PathBuf::from(filename);
    let basename = path
        .file_stem()
        .context("failed when requiring file stem")?
        .to_str()
        .context("failed when parsing basename to string")?;
    let block = Parser::from(stream).parse_file(basename.into(), path, father);
    Ok(block)
}

// pub fn complie_string(code: &str, father: Option<&Block>) -> Result<Vec<Block>> {
//     let cursor = Cursor::new(code.as_bytes());
//     let stream = Lexer::from(cursor).into_token_stream();
//     let block = Parser::from(stream).parse_file(uuid::Uuid::new_v4().to_string(), father);
//     Ok(block)
// }
