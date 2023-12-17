use crate::core::{bytecode::ByteCode, value::Value};

use super::namespace::NameSpace;

#[derive(Debug, Clone)]
pub enum BlockType {
    Module, // dir
    File,   // file
    Fn,
    // TODO: supprot class
    // Class,
}

/// wrapper for bytecodes
#[derive(Debug, Clone)]
pub struct Block {
    pub t: BlockType,
    pub name: String,
    pub full_name: String,
    pub args: Vec<String>,
    pub byte_codes: Vec<ByteCode>,
    pub constants: Vec<Value>,
    pub locals: Vec<String>,
    pub sub_blocks: Vec<Block>,
    pub pc: usize,
}

impl Block {
    pub fn new(full_name: String, t: BlockType, args: Vec<String>) -> Self {
        let name = full_name.split("::").last().unwrap();
        Self {
            t,
            name: name.into(),
            full_name,
            args,
            byte_codes: vec![],
            constants: vec![],
            locals: vec![],
            sub_blocks: vec![],
            pc: 0,
        }
    }

    pub fn inherite(father: &Block, name: String, t: BlockType, args: Vec<String>) -> Self {
        let full_name = father.full_name.clone() + "::" + &name;
        Self {
            t,
            name,
            full_name,
            args,
            byte_codes: father.byte_codes.clone(),
            constants: father.constants.clone(),
            locals: father.locals.clone(),
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

    pub fn jump_block(&mut self) {
        let mut count = 0;
        loop {
            let code = self.go_ahead();
            match code {
                Some(&ByteCode::EnterBlock) => count += 1,
                Some(&ByteCode::LeaveBlock) => {
                    count -= 1;
                    if count == 0 {
                        break;
                    }
                }
                Some(_) => {}
                None => panic!("unexpected eos!"),
            }
        }
    }
}
