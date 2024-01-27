use crate::core::bytecode::ByteCode;

use super::{bytecode::Decorate, namespace::NameSpace, typ::Type};

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
    pub args: Vec<(String, Type)>,
    pub byte_codes: Vec<ByteCode>,
    #[serde(skip)]
    pub sub_blocks: Vec<Block>,
    #[serde(skip)]
    #[serde(default)]
    pub pc: usize,
}

impl Block {
    pub fn new(full_name: String, t: BlockType, args: Vec<(String, Type)>) -> Self {
        let name = full_name.split("::").last().unwrap();
        Self {
            t,
            name: name.into(),
            full_name,
            args,
            byte_codes: vec![],
            sub_blocks: vec![],
            pc: 0,
        }
    }

    pub fn inherite(father: &Block, name: String, t: BlockType, args: Vec<(String, Type)>) -> Self {
        let full_name = father.full_name.clone() + "::" + &name;
        Self {
            t,
            name,
            full_name,
            args,
            byte_codes: father.byte_codes.clone(),
            sub_blocks: vec![],
            pc: 0,
        }
    }

    pub fn namespace(&self) -> NameSpace {
        NameSpace::new(self.full_name.clone()).get_super()
    }

    pub fn add_sub_block(&mut self, block: Block) {
        self.sub_blocks.push(block);
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
                    Decorate::Fog => {}
                },
                Some(_) => {}
                None => panic!("unexpected eos!"),
            }
        }
    }
}
