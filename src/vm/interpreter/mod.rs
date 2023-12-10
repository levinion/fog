mod func;
mod wrapper;

use std::collections::{HashMap, VecDeque};

use crate::{
    complier::{block::Block, ir::IR},
    core::{bytecode::ByteCode, value::Value},
};

use self::func::init_global_table;

pub struct Interpreter<'a> {
    stack: VecDeque<Value>,
    global_table: HashMap<String, Value>,
    local_table: HashMap<String, Value>,
    ir: &'a IR,
}

impl<'a> Interpreter<'a> {
    pub fn new(ir: &'a IR) -> Self {
        let global_table = init_global_table();
        let local_table = HashMap::new();
        Self {
            stack: VecDeque::new(),
            global_table,
            local_table,
            ir,
        }
    }

    pub fn execute(&mut self, mut block: Block) {
        while let Some(code) = block.go_ahead() {
            match *code {
                ByteCode::GetGlobal => self.get_global(),
                ByteCode::LoadConst(index) => self.load_const(&mut block, index),
                ByteCode::CallSuperFunction(argc) => self.call_super_function(argc),
                ByteCode::CallFunction(argc) => self.call_function(argc),
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
