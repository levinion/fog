use anyhow::{anyhow, Result};

use crate::core::{
    bytecode::FunctionType,
    value::{Args, Function, Value},
};

use super::Interpreter;

impl Interpreter {
    pub async fn call_function(&mut self, argc: usize, t: FunctionType) -> Result<()> {
        let args = self.collect_args(argc);
        if let Value::Function(func) = self.stack.pop_back().unwrap() {
            match func {
                Function::MetaFunction(meta) => match t {
                    FunctionType::NormalFunction => {
                        meta(args);
                    }
                    FunctionType::FogFunction => {
                        tokio::spawn(async move {
                            meta(args);
                        });
                    }
                    FunctionType::Undefined => unreachable!(),
                },
                Function::NormalFunction(block) => match t {
                    FunctionType::NormalFunction => {
                        let mut new_interpreter = Interpreter::new();
                        new_interpreter.execute(block, args).await?;
                    }
                    FunctionType::FogFunction => {
                        let mut new_interpreter = Interpreter::new();
                        tokio::spawn(async move {
                            new_interpreter.execute(block, args).await.unwrap();
                        });
                    }
                    FunctionType::Undefined => unreachable!(),
                },
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

    pub fn call_method(&mut self, argc: usize) -> Result<()> {
        let args = self.collect_args(argc);
        // method name
        let name = if let Value::String(name) = self.stack.pop_back().unwrap() {
            name
        } else {
            return Err(anyhow!("invalid function name type"));
        };
        let value = self.stack.pop_back().unwrap();
        Ok(())
    }
}
