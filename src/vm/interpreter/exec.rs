use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::core::{
    block::Block,
    value::{Args, Function, Value},
};

use super::Interpreter;

impl Interpreter {
    pub async fn call_function(&mut self, argc: usize, block: Arc<Block>) -> Result<()> {
        if let Value::Function(func) = self.stack.pop_back().unwrap() {
            let args = self.collect_args(argc);
            match func {
                Function::MetaFunction(meta) => {
                    let r = meta.0(args, &block);
                    self.stack.push_back(r?);
                }
                Function::NormalFunction(block) => {
                    let mut new_interpreter = Interpreter::new();
                    let r = new_interpreter.execute(block, args).await?;
                    self.stack.push_back(r);
                }
            };
            Ok(())
        } else {
            Err(anyhow!("a function is needed!"))
        }
    }

    pub async fn fog_call_function(&mut self, argc: usize, block: Arc<Block>) -> Result<()> {
        if let Value::Function(func) = self.stack.pop_back().unwrap() {
            let args = self.collect_args(argc);
            match func {
                Function::MetaFunction(meta) => {
                    tokio::spawn(async move {
                        meta.0(args, &block).unwrap();
                    });
                }
                Function::NormalFunction(block) => {
                    let mut new_interpreter = Interpreter::new();
                    tokio::spawn(async move {
                        new_interpreter.execute(block, args).await.unwrap();
                    });
                }
            }
            Ok(())
        } else {
            Err(anyhow!("a function is needed!"))
        }
    }

    fn collect_args(&mut self, argc: usize) -> Args {
        // collect args
        let mut args = vec![];
        for _ in 0..argc {
            args.push(self.stack.pop_back().unwrap());
        }
        args.reverse();
        args
    }
}
