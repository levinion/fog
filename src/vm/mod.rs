mod interpreter;
use std::{collections::HashMap, sync::Arc};

use interpreter::Interpreter;

use crate::core::{block::Block, ir::IR};

pub struct VM {
    block_table: Arc<HashMap<String, Block>>,
}

impl VM {
    pub fn new(ir: IR) -> Self {
        Self {
            block_table: Arc::new(ir.into()),
        }
    }

    pub async fn execute(&mut self) {
        let mut itpt = Interpreter::new(self.block_table.clone());
        let main_block = self.block_table.get("main").unwrap();
        itpt.execute(main_block.clone()).await;
    }
}
