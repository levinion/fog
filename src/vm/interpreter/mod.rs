mod exec;
mod meta;
mod op;
mod wrapper;

use std::collections::{HashMap, VecDeque};

use anyhow::Result;

use crate::core::{
    block::Block,
    bytecode::ByteCode,
    value::{Args, Value},
};

use self::meta::init_global_table;

use super::manager::Manager;

/// An Interpreter is an instance that executes a block.
pub struct Interpreter {
    stack: VecDeque<Value>,
    global_table: HashMap<String, Value>,
    local_table: HashMap<String, Value>,
    manager: Manager,
}

impl Interpreter {
    pub fn new(manager: Manager) -> Self {
        let global_table = init_global_table();
        let local_table = HashMap::new();
        Self {
            stack: VecDeque::new(),
            global_table,
            local_table,
            manager,
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
                ByteCode::GetGlobal => self.get_global(),
                ByteCode::LoadConst(index) => self.load_const(&mut block, index),
                ByteCode::CallMetaFunction(argc) => self.call_meta_function(argc)?,
                ByteCode::CallFogFunction(argc) => self.call_fog_function(&block, argc).await?,
                ByteCode::CallFunction(argc) => self.call_function(&block, argc).await?,
                ByteCode::StoreLocal(index) => self.store_local(&mut block, index),
                ByteCode::LoadLocal(index) => self.load_local(&mut block, index),
                ByteCode::JumpIfFalse => self.jump_if_false(&mut block),
                ByteCode::UnaryOP(op) => self.unary_op(op)?,
                ByteCode::BinaryOP(op) => self.binary_op(op)?,
                ByteCode::EnterBlock | ByteCode::LeaveBlock => {
                    //do nothing, this is only a flag
                }
            }
        }
        Ok(())
    }
}
