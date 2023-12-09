mod func;
mod wrapper;

use std::collections::{HashMap, VecDeque};

use crate::{
    core::{bytecode::ByteCode, value::Value},
    parse::ir::IR,
};

use self::func::init_global_table;

pub struct VM {
    stack: VecDeque<Value>,
    global_table: HashMap<String, Value>,
    local_table: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        let global_table = init_global_table();
        let local_table = HashMap::new();
        Self {
            stack: VecDeque::new(),
            global_table,
            local_table,
        }
    }

    pub fn execute(&mut self, mut ir: IR) {
        while let Some(code) = ir.go_ahead() {
            match *code {
                ByteCode::GetGlobal => self.get_global(),
                ByteCode::LoadConst(index) => self.load_const(&mut ir, index),
                ByteCode::CallFunction(argc) => self.call_function(argc),
                ByteCode::StoreLocal(index) => self.store_local(&mut ir, index),
                ByteCode::LoadLocal(index) => self.load_local(&mut ir, index),
                ByteCode::JumpIfFalse => self.jump_if_false(&mut ir),
                ByteCode::UnaryOP(op) => self.unary_op(op),
                ByteCode::BinaryOP(op) => self.binary_op(op),
                ByteCode::EnterBlock | ByteCode::LeaveBlock => {
                    //do nothing, this is only a flag
                }
            }
        }
    }
}
