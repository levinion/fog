use anyhow::{anyhow, Result};

use crate::{
    core::{
        block::Block,
        value::{Function, Value},
    },
    vm::runtime::global::GlobalItem,
    VM,
};

use super::Interpreter;

impl Interpreter {
    async fn get_global(&mut self, block: &Block, s: &str) -> bool {
        let item = VM
            .get()
            .unwrap()
            .runtime
            .get_global_by_name(s, block.namespace());
        match item {
            Some(GlobalItem::Meta(meta)) => self
                .stack
                .push_back(Value::Function(Function::MetaFunction(*meta))),
            Some(GlobalItem::Block(block)) => self
                .stack
                .push_back(Value::Function(Function::NormalFunction(block.clone()))),
            None => return false,
        }
        true
    }

    pub async fn load_name(&mut self, block: &Block) -> Result<()> {
        if let Value::Name(s) = self.stack.pop_back().unwrap() {
            if self.load_local(&s) {
                return Ok(());
            }
            if self.get_global(block, &s).await {
                Ok(())
            } else {
                Err(anyhow!("undefined name!"))
            }
        } else {
            Err(anyhow!("a name is needed!"))
        }
    }

    //  load a value to the stack
    pub fn load_value(&mut self, value: Value) {
        self.stack.push_back(value);
    }

    // [name | value] -> into local_table
    pub fn store_local(&mut self) {
        let value = self.stack.pop_back().unwrap();
        let name = self.stack.pop_back().unwrap();
        if let Value::Name(s) = name {
            self.local_table.insert(s, value);
        } else {
            panic!("a name is needed! found: {:?}", name)
        }
    }

    // take a name, and load the value.
    fn load_local(&mut self, s: &str) -> bool {
        let value = self.local_table.get(s);
        match value {
            Some(v) => {
                self.stack.push_back(v.clone());
                true
            }
            None => false,
        }
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
