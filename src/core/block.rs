use std::{path::PathBuf, sync::Arc};

use crate::core::bytecode::ByteCode;

use super::{namespace::NameSpace, value::Type};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockType {
    Module, // dir
    File,   // file
    Fn,
    // TODO: supprot class
    // Class,
}

/// wrapper for bytecodes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
pub struct Block {
    #[serde(rename = "type")]
    pub t: BlockType,
    pub name: String,
    pub full_name: String,
    pub path: PathBuf,
    pub args: Vec<(Arc<String>, Type)>,
    pub byte_codes: Vec<ByteCode>,
}

impl Block {
    pub fn new(
        full_name: String,
        path: PathBuf,
        t: BlockType,
        args: Vec<(Arc<String>, Type)>,
    ) -> Self {
        let name = full_name.split("::").last().unwrap();
        Self {
            t,
            name: name.into(),
            full_name,
            path,
            args,
            byte_codes: vec![],
        }
    }

    pub fn inherite(
        father: &Block,
        name: String,
        path: PathBuf,
        t: BlockType,
        args: Vec<(Arc<String>, Type)>,
    ) -> Self {
        let full_name = father.full_name.clone() + "::" + &name;
        Self {
            t,
            name,
            full_name,
            path,
            args,
            byte_codes: father.byte_codes.clone(),
        }
    }

    pub fn namespace(&self) -> NameSpace {
        NameSpace::new(self.full_name.clone()).get_super()
    }
}
