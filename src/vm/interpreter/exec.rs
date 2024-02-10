use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::core::{
    block::Block,
    value::{Args, Function, Value},
};

use super::Interpreter;

impl Interpreter {
    pub async fn call_function(&mut self, argc: usize, block: Arc<Block>) -> Result<()> {
        let args = self.collect_args(argc);
        if let Value::Function(func) = self.stack.pop_back().unwrap() {
            match func {
                Function::MetaFunction(meta) => {
                    let r = meta(args, &block);
                    self.stack.push_back(r?);
                }
                Function::NormalFunction(block) => {
                    let mut new_interpreter = Interpreter::new();
                    new_interpreter.execute(block, args).await?;
                }
            };
            Ok(())
        } else {
            Err(anyhow!("a function is needed!"))
        }
    }

    pub async fn fog_call_function(&mut self, argc: usize, block: Arc<Block>) -> Result<()> {
        let args = self.collect_args(argc);
        if let Value::Function(func) = self.stack.pop_back().unwrap() {
            match func {
                Function::MetaFunction(meta) => {
                    tokio::spawn(async move {
                        meta(args, &block).unwrap();
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

    // alias of call_function
    // a.b(...args) = b(a,...args) if fn b(self: a, ...args)
    pub fn call_method(&mut self, argc: usize) -> Result<()> {
        // let args = self.collect_args(argc);
        // // method name
        // let name = if let Value::String(name) = self.stack.pop_back().unwrap() {
        //     name
        // } else {
        //     return Err(anyhow!("invalid function name type"));
        // };
        // let value = self.stack.pop_back().unwrap();
        Ok(())
    }
}
