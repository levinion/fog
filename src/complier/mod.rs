mod lexer;
pub mod optimizer;
mod parser;

use anyhow::{Context, Result};

use crate::core::{
    block::{Block, BlockType},
    ir::IR,
};

use self::{lexer::Lexer, parser::Parser};
use std::{fs::File, path::PathBuf};

pub fn complie(root: &str) -> Result<IR> {
    // walk for all files under root
    Ok(handle_modules(root)?.into())
}

fn handle_modules(root: &str) -> Result<Vec<Block>> {
    let mut modules = vec![];
    for entry in std::fs::read_dir(root)? {
        let path = entry?.path();
        let name = path.to_str().context("cannot parse this path to string")?;
        let basename = path
            .file_name()
            .context("failed when requiring basename")?
            .to_str()
            .context("failed to parse basename to string")?;

        if path.is_dir() {
            let mut module = Block::new(basename.into(), BlockType::Module, vec![]);
            module = handle_module(name, module)?;
            modules.push(module);
        } else {
            let block = complie_file(name, None)?;
            modules.push(block);
        }
    }
    Ok(modules)
}

fn handle_module(root: &str, mut module: Block) -> Result<Block> {
    for entry in std::fs::read_dir(root)? {
        let path = entry?.path();
        let name = path.to_str().context("invalid path")?;
        let basename = path
            .file_name()
            .context("failed when requiring basename")?
            .to_str()
            .context("failed to parse basename to string")?;
        if path.is_dir() {
            let mut new_module =
                Block::inherite(&module, basename.into(), BlockType::Module, vec![]);
            new_module = handle_module(name, new_module)?;
            module.add_sub_block(new_module);
        } else {
            let block = complie_file(name, Some(&module))?;
            module.add_sub_block(block);
        }
    }
    Ok(module)
}

pub fn complie_file(filename: &str, father: Option<&Block>) -> Result<Block> {
    let file = File::open(filename)?;
    let stream = Lexer::from(file).into_token_stream();
    let path = PathBuf::from(filename);
    let basename = path
        .file_stem()
        .context("failed when requiring file stem")?
        .to_str()
        .context("failed when parsing basename to string")?;
    let block = Parser::from(stream).parse_file(basename.into(), father);
    Ok(block)
}
