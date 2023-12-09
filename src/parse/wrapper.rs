use crate::core::{bytecode::ByteCode, value::Value};

use super::Parser;

impl Parser {
    /// load a value to stack
    pub fn load_const(&mut self, value: Value) {
        // add argument name to constants table and then load it to stack

        // if there exists the value, then just return it.
        if let Some(index) = self.constants.iter().position(|x| x == &value) {
            self.byte_codes.push(ByteCode::LoadConst(index));
            return;
        }
        self.constants.push(value);
        self.byte_codes
            .push(ByteCode::LoadConst(self.constants.len() - 1));
    }

    // get from global and load to the stack
    pub fn get_global(&mut self) {
        self.byte_codes.push(ByteCode::GetGlobal);
    }

    // take the function and args then call it.
    pub fn call(&mut self, argc: usize) {
        self.byte_codes.push(ByteCode::CallFunction(argc));
    }

    // store the name to locals and return its index
    pub fn store_local(&mut self, name: String) {
        // if there exists the value, then just return it.
        if let Some(index) = self.locals.iter().position(|x| x == &name) {
            self.byte_codes.push(ByteCode::StoreLocal(index));
            return;
        }

        self.locals.push(name);
        self.byte_codes
            .push(ByteCode::StoreLocal(self.locals.len() - 1));
    }

    // load local variable from the locals
    pub fn load_local(&mut self, name: String) {
        let index = self
            .locals
            .iter()
            .rposition(|x| *x == name)
            .unwrap_or_else(|| panic!("name not found: {name}"));
        self.byte_codes.push(ByteCode::LoadLocal(index));
    }

    pub fn jump_if_false(&mut self) {
        self.byte_codes.push(ByteCode::JumpIfFalse);
    }

    pub fn enter_block(&mut self) {
        self.byte_codes.push(ByteCode::EnterBlock);
    }

    pub fn leave_block(&mut self) {
        self.byte_codes.push(ByteCode::LeaveBlock);
    }
}
