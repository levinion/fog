mod exec;
mod op;
mod wrapper;

use std::collections::{HashMap, VecDeque};

use anyhow::Result;

use crate::core::{
    block::Block,
    bytecode::ByteCode,
    value::{Args, Value},
};

/// An Interpreter is an instance that executes a block.
pub struct Interpreter {
    stack: VecDeque<Value>,
    local_table: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            local_table: HashMap::new(),
        }
    }

    #[async_recursion::async_recursion]
    // args: args read by caller
    pub async fn execute(&mut self, mut block: Block, args: Args) -> Result<()> {
        // handle args here
        block
            .args
            .iter()
            .zip(args.into_iter())
            .for_each(|(name, arg)| {
                self.local_table.insert(name.clone(), arg);
            });

        while let Some(code) = block.go_ahead() {
            match *code {
                ByteCode::GetGlobal => self.get_global().await,
                ByteCode::LoadConst(index) => self.load_const(&mut block, index),
                ByteCode::CallFunction(argc, t) => {
                    self.call_function(&block, argc, t.into()).await?
                }
                ByteCode::CallMethod(argc) => self.call_method(argc)?,
                ByteCode::StoreLocal(index) => self.store_local(&mut block, index),
                ByteCode::LoadLocal(index) => self.load_local(&mut block, index),
                ByteCode::JumpIfFalse => self.jump_if_false(&mut block),
                ByteCode::UnaryOP(op) => self.unary_op(op)?,
                ByteCode::BinaryOP(op) => self.binary_op(op)?,
                ByteCode::Decorate(_) => {
                    panic!("decorate should be optimized!")
                }
            }
        }
        Ok(())
    }
}
