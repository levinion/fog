use crate::{
    core::{block::Block, value::Value},
    VM,
};

use super::Interpreter;

impl Interpreter {
    /// take a element then get global variable, usually a function
    pub async fn get_global(&mut self) {
        if let Value::String(s) = self.stack.pop_back().unwrap() {
            let vm = VM.lock().await;
            let func = vm.runtime.get_global(&s).unwrap_or(&Value::None).clone();
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
