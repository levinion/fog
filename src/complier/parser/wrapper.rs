use crate::core::{block::Block, bytecode::ByteCode, value::Value};

use super::Parser;

impl Parser {
    /// load a value to stack
    pub fn load_const(&mut self, block: &mut Block, value: Value) {
        // add argument name to constants table and then load it to stack

        // if there exists the value, then just return it.
        if let Some(index) = block.constants.iter().position(|x| x == &value) {
            block.byte_codes.push(ByteCode::LoadConst(index));
            return;
        }
        block.constants.push(value);
        block
            .byte_codes
            .push(ByteCode::LoadConst(block.constants.len() - 1));
    }

    // get from global and load to the stack
    pub fn get_global(&mut self, block: &mut Block) {
        block.byte_codes.push(ByteCode::GetGlobal);
    }

    // take the function and args then call it.
    pub fn call_super(&mut self, block: &mut Block, argc: usize) {
        block.byte_codes.push(ByteCode::CallMetaFunction(argc));
    }

    // take the function and args then call it.
    pub fn call(&mut self, block: &mut Block, argc: usize) {
        block.byte_codes.push(ByteCode::CallFunction(argc));
    }

    // take the function and args then call it.
    pub fn call_fog(&mut self, block: &mut Block, argc: usize) {
        block.byte_codes.push(ByteCode::CallFogFunction(argc));
    }

    // store the name to locals and return its index
    pub fn store_local(&mut self, block: &mut Block, name: String) {
        // if there exists the value, then just return it.
        if let Some(index) = block.locals.iter().position(|x| x == &name) {
            block.byte_codes.push(ByteCode::StoreLocal(index));
            return;
        }

        block.locals.push(name);
        block
            .byte_codes
            .push(ByteCode::StoreLocal(block.locals.len() - 1));
    }

    // load local variable from the locals
    pub fn load_local(&mut self, block: &mut Block, name: String) {
        let index = block
            .locals
            .iter()
            .rposition(|x| *x == name)
            .unwrap_or_else(|| panic!("name not found: {name}"));
        block.byte_codes.push(ByteCode::LoadLocal(index));
    }

    pub fn jump_if_false(&mut self, block: &mut Block) {
        block.byte_codes.push(ByteCode::JumpIfFalse);
    }

    pub fn enter_block(&mut self, block: &mut Block) {
        block.byte_codes.push(ByteCode::EnterBlock);
    }

    pub fn leave_block(&mut self, block: &mut Block) {
        block.byte_codes.push(ByteCode::LeaveBlock);
    }
}
