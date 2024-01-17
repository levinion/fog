use anyhow::{anyhow, Result};

use crate::{
    core::{
        block::Block,
        bytecode::FunctionType,
        token::IsMeta,
        value::{Args, Value},
    },
    VM,
};

use super::Interpreter;

impl Interpreter {
    pub async fn call_function(
        &mut self,
        block: &Block,
        argc: usize,
        t: FunctionType,
    ) -> Result<()> {
        let args = self.collect_args(argc);
        if let Value::String(name) = self.stack.pop_back().unwrap() {
            // handle normal function call and fog function call.
            let mut vm = VM.lock().await;
            match t {
                FunctionType::FogFunction => {
                    if name.is_meta() {
                        vm.runtime.get_global(&name).unwrap();
                    }
                    vm.runtime.par_exec(&name, args, block.namespace()).await?;
                }
                FunctionType::NormalFunction => {
                    vm.runtime.exec(&name, args, block.namespace()).await?;
                }
                FunctionType::Undefined => panic!("optimizer error: undefined function type!"),
            }
        } else {
            return Err(anyhow!("invalid function name type"));
        }
        Ok(())
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
