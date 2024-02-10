mod exec;
mod op;
mod wrapper;

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use anyhow::Result;

use crate::core::{
    block::Block,
    bytecode::ByteCode,
    value::{Args, Value},
};

/// An Interpreter is an instance that executes a block.
pub struct Interpreter {
    pub stack: VecDeque<Value>,
    pub local_table: HashMap<String, Value>,
    pub pc: isize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            local_table: HashMap::new(),
            pc: 0,
        }
    }

    #[async_recursion::async_recursion]
    // args: args read by caller
    pub async fn execute(&mut self, block: Arc<Block>, args: Args) -> Result<()> {
        // handle args here
        block
            .args
            .iter()
            .zip(args.into_iter())
            .for_each(|(name, arg)| {
                self.local_table.insert(name.0.clone(), arg);
            });

        while let Some(code) = self.go_ahead(&block) {
            match code.clone() {
                ByteCode::CallFunction(argc) => {
                    self.call_function(argc, Arc::clone(&block)).await?
                }
                ByteCode::FogCallFunction(argc) => {
                    self.fog_call_function(argc, Arc::clone(&block)).await?
                }
                ByteCode::CallMethod(argc) => self.call_method(argc)?,
                ByteCode::LoadValue(value) => self.load_value(value),
                ByteCode::StoreLocal => self.store_local(),
                ByteCode::LoadName => self.load_name(&block).await?,
                ByteCode::UnaryOP(op) => self.unary_op(op)?,
                ByteCode::BinaryOP(op) => self.binary_op(op)?,
                ByteCode::JumpIfFalse(n) => self.jump_if_false(n),
                ByteCode::Jump(n) => self.jump(n),
                // ByteCode::Decorate(_) => {
                //     panic!("decorate should be optimized!")
                // }
            }
        }
        Ok(())
    }

    fn go_ahead<'a>(&'a mut self, block: &'a Block) -> Option<&ByteCode> {
        let r = block.byte_codes.get(self.pc as usize);
        self.pc += 1;
        r
    }
}
