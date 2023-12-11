use crate::core::{block::Block, value::Value};

use super::Interpreter;

impl<'a> Interpreter<'a> {
    /// take a element then get global variable, usually a function
    pub fn get_global(&mut self) {
        if let Value::String(s) = self.stack.pop_back().unwrap() {
            let func = self.global_table.get(&s).unwrap_or(&Value::None).clone();
            self.stack.push_back(func);
        } else {
            panic!("panic when get global!")
        }
    }

    //  load a value to the stack
    pub fn load_const(&mut self, block: &mut Block, index: usize) {
        self.stack.push_back(block.constants[index].clone());
    }

    // take a constant, bind it with a name, then set it as a local value.
    pub fn store_local(&mut self, block: &mut Block, index: usize) {
        let value = self.stack.pop_back().unwrap();
        let name = block.locals.get(index).unwrap().clone();
        self.local_table.insert(name, value);
    }

    // take a name, and load the value.
    pub fn load_local(&mut self, block: &mut Block, index: usize) {
        let name = block.locals.get(index).unwrap().clone();
        self.stack
            .push_back(self.local_table.get(&name).unwrap().clone());
    }

    pub fn jump_if_false(&mut self, block: &mut Block) {
        let b = if let Value::Bool(b) = self.stack.pop_back().unwrap() {
            b
        } else {
            panic!("expected bool!")
        };
        if !b {
            block.jump_block();
        }
    }
}
