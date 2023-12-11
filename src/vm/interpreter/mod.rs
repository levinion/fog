mod exec;
mod meta;
mod op;
mod wrapper;

use std::collections::{HashMap, VecDeque};

use crate::core::{block::Block, bytecode::ByteCode, value::Value};

use self::meta::init_global_table;

pub struct Interpreter<'a> {
    stack: VecDeque<Value>,
    global_table: HashMap<String, Value>,
    local_table: HashMap<String, Value>,
    block_table: &'a HashMap<String, Block>,
}

impl<'a> Interpreter<'a> {
    pub fn new(block_table: &'a HashMap<String, Block>) -> Self {
        let global_table = init_global_table();
        let local_table = HashMap::new();
        Self {
            stack: VecDeque::new(),
            global_table,
            local_table,
            block_table,
        }
    }

    #[async_recursion::async_recursion]
    pub async fn execute(&mut self, mut block: Block) {
        while let Some(code) = block.go_ahead() {
            match *code {
                ByteCode::GetGlobal => self.get_global(),
                ByteCode::LoadConst(index) => self.load_const(&mut block, index),
                ByteCode::CallMetaFunction(argc) => self.call_meta_function(argc),
                ByteCode::CallFogFunction(argc) => self.call_fog_function(argc).await,
                ByteCode::CallFunction(argc) => self.call_function(argc).await,
                ByteCode::StoreLocal(index) => self.store_local(&mut block, index),
                ByteCode::LoadLocal(index) => self.load_local(&mut block, index),
                ByteCode::JumpIfFalse => self.jump_if_false(&mut block),
                ByteCode::UnaryOP(op) => self.unary_op(op),
                ByteCode::BinaryOP(op) => self.binary_op(op),
                ByteCode::EnterBlock | ByteCode::LeaveBlock => {
                    //do nothing, this is only a flag
                }
            }
        }
    }
}
