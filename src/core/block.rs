use std::path::PathBuf;

use crate::core::bytecode::ByteCode;

use super::{bytecode::Decorate, namespace::NameSpace, value::Type};

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
    pub args: Vec<(String, Type)>,
    pub byte_codes: Vec<ByteCode>,
    #[serde(skip)]
    #[serde(default)]
    pub pc: usize,
}

impl Block {
    pub fn new(full_name: String, path: PathBuf, t: BlockType, args: Vec<(String, Type)>) -> Self {
        let name = full_name.split("::").last().unwrap();
        Self {
            t,
            name: name.into(),
            full_name,
            path,
            args,
            byte_codes: vec![],
            pc: 0,
        }
    }

    pub fn inherite(
        father: &Block,
        name: String,
        path: PathBuf,
        t: BlockType,
        args: Vec<(String, Type)>,
    ) -> Self {
        let full_name = father.full_name.clone() + "::" + &name;
        Self {
            t,
            name,
            full_name,
            path,
            args,
            byte_codes: father.byte_codes.clone(),
            pc: 0,
        }
    }

    pub fn namespace(&self) -> NameSpace {
        NameSpace::new(self.full_name.clone()).get_super()
    }

    pub fn go_ahead(&mut self) -> Option<&ByteCode> {
        let code = self.byte_codes.get(self.pc);
        self.pc += 1;
        code
    }

    pub fn reset_pc(&mut self) {
        self.pc = 0;
    }

    pub fn jump_block(&mut self) {
        let mut count = 0;
        loop {
            let code = self.go_ahead();
            match code {
                Some(&ByteCode::Decorate(decorate)) => match decorate {
                    Decorate::EnterBlock => count += 1,
                    Decorate::LeaveBlock => {
                        count -= 1;
                        if count == 0 {
                            break;
                        }
                    }
                },
                Some(_) => {}
                None => panic!("unexpected eos!"),
            }
        }
    }
}
