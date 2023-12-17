use std::{collections::HashMap, sync::Arc};

use crate::core::{block::Block, ir::IR, namespace::NameSpace, value::Args};

use super::interpreter::Interpreter;

#[derive(Clone)]
pub struct Manager(Arc<HashMap<String, Block>>);

impl Manager {
    pub fn new(ir: IR) -> Self {
        Manager(Arc::new(ir.into()))
    }

    pub async fn exec(&self, name: &str, args: Args, namespace: NameSpace) {
        let mut itpr = Interpreter::new(self.clone());
        let main_block = self.get_block_by_name(name, namespace);
        itpr.execute(main_block.clone(), args).await;
    }

    pub async fn par_exec(&self, name: &str, args: Args, namespace: NameSpace) {
        let block = self.get_block_by_name(name, namespace);
        let manager = self.clone();
        tokio::spawn(async move {
            let mut new_interpreter = Interpreter::new(manager);
            new_interpreter.execute(block, args).await;
        });
    }

    /// find the true block by name and its father's namespace.
    // TODO: add other namespace: std, dependencies...
    fn get_block_by_name(&self, name: &str, namespace: NameSpace) -> Block {
        let relative_name = namespace.append(name).to_string();

        #[allow(clippy::single_element_loop)]
        for name in [&relative_name] {
            let block = self.0.get(name);
            if let Some(block) = block {
                return block.clone();
            }
        }

        if let Some(block) = self.0.get(name) {
            return block.clone();
        }
        panic!("name not found: {}", name);
    }
}
