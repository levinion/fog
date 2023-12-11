use std::{collections::HashMap, sync::Arc};

use crate::core::{block::Block, ir::IR, value::Args};

use super::interpreter::Interpreter;

#[derive(Clone)]
pub struct Manager(Arc<HashMap<String, Block>>);

impl Manager {
    pub fn new(ir: IR) -> Self {
        Manager(Arc::new(ir.into()))
    }

    pub async fn exec(&self, name: &str, args: Args) {
        let mut itpr = Interpreter::new(self.clone());
        let main_block = self.get_block_by_name(name);
        itpr.execute(main_block.clone(), args).await;
    }

    pub async fn par_exec(&self, name: &str, args: Args) {
        let block = self.get_block_by_name(name);
        let manager = self.clone();
        tokio::spawn(async move {
            let mut new_interpreter = Interpreter::new(manager);
            new_interpreter.execute(block, args).await;
        });
    }

    // get block by name
    // name -> main::name -> ?
    pub fn get_block_by_name(&self, name: &str) -> Block {
        self.0
            .get(name)
            .unwrap_or_else(|| {
                let name = "main::".to_string() + name;
                self.0.get(&name).unwrap()
            })
            .clone()
    }
}
