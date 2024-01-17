mod global;
mod method;

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use tokio::sync::Mutex;

use crate::core::{
    block::Block,
    ir::IR,
    namespace::NameSpace,
    typ::Type,
    value::{Args, Value},
};

use self::method::init_method_table;

use super::interpreter::Interpreter;

pub struct Runtime {
    interpreters: Mutex<HashMap<String, Interpreter>>,
    block_table: Option<HashMap<String, Block>>,
    global_table: HashMap<String, Value>,
    method_table: HashMap<Type, fn(Args) -> i32>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            interpreters: Mutex::new(HashMap::new()),
            block_table: None, // other blocks defined here, including custom functions
            global_table: HashMap::new(), // global/meta functions defined here
            method_table: init_method_table(),
        }
    }

    pub fn set_ir(&mut self, ir: IR) -> &mut Self {
        self.block_table = Some(ir.optimize());
        self
    }

    pub async fn exec(&mut self, name: &str, args: Args, namespace: NameSpace) -> Result<()> {
        let id = self.build_new_interpreter().await;
        let mut new_interpreter = self.interpreters.lock().await;
        let new_interpreter = new_interpreter.get_mut(&id).unwrap();
        let main_block = self.get_block_by_name(name, namespace)?;
        new_interpreter.execute(main_block.clone(), args).await
    }

    pub async fn par_exec(&self, name: &str, args: Args, namespace: NameSpace) -> Result<()> {
        let block = self.get_block_by_name(name, namespace)?;
        tokio::spawn(async move {
            let mut new_interpreter = Interpreter::new();
            new_interpreter.execute(block, args).await.unwrap();
        });
        Ok(())
    }

    /// find the true block by name and its father's namespace.
    // TODO: add other namespace: std, dependencies...
    fn get_block_by_name(&self, name: &str, namespace: NameSpace) -> Result<Block> {
        let relative_name = namespace.append(name).to_string();

        #[allow(clippy::single_element_loop)]
        for name in [&relative_name] {
            let block = self.block_table.as_ref().unwrap().get(name);
            if let Some(block) = block {
                return Ok(block.clone());
            }
        }

        if let Some(block) = self.block_table.as_ref().unwrap().get(name) {
            return Ok(block.clone());
        }
        Err(anyhow!("name not found: {}", name))
    }

    async fn build_new_interpreter(&mut self) -> String {
        let uid = uuid::Uuid::new_v4().to_string();
        self.interpreters
            .lock()
            .await
            .insert(uid.clone(), Interpreter::new());
        uid
    }

    pub fn get_global(&self, key: &str) -> Option<&Value> {
        self.global_table.get(key)
    }
}
