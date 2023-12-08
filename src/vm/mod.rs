mod func;
use std::collections::{HashMap, VecDeque};

use crate::{bytecode::ByteCode, parse::Parser, value::Value};

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

    pub fn execute(&mut self, mut parser: Parser) {
        while let Some(code) = parser.byte_codes.pop_front() {
            match code {
                // take a element then get global variable, usually a function
                ByteCode::GetGlobal => {
                    if let Value::String(s) = self.stack.pop_back().unwrap() {
                        let func = self.global_table.get(&s).unwrap_or(&Value::None).clone();
                        self.stack.push_back(func);
                    } else {
                        panic!("panic when get global!")
                    }
                }

                //  load a value to the stack
                ByteCode::LoadConst { index } => {
                    self.stack.push_back(parser.constants[index].clone());
                }

                // take a function name constant and args, call the function.
                ByteCode::CallFunction { argc } => {
                    // collect args
                    let mut args = vec![];
                    for _ in 0..argc {
                        args.push(self.stack.pop_back().unwrap());
                    }
                    args.reverse();

                    // get function
                    if let Value::Fn(func) = self.stack.pop_back().unwrap() {
                        func(args);
                    }
                }

                // take a constant, bind it with a name, then set it as a local value.
                ByteCode::StoreLocal { index } => {
                    let value = self.stack.pop_back().unwrap();
                    let name = parser.locals.get(index).unwrap().clone();
                    self.local_table.insert(name, value);
                }

                // take a name, and load the value.
                ByteCode::LoadLocal { index } => {
                    let name = parser.locals.get(index).unwrap().clone();
                    self.stack
                        .push_back(self.local_table.get(&name).unwrap().clone());
                }

                ByteCode::JumpIfFalse => {
                    let b = if let Value::Bool(b) = self.stack.pop_back().unwrap() {
                        b
                    } else {
                        panic!("expected bool!")
                    };
                    if !b {
                        //TODO: solve nesting blocks
                        while parser.byte_codes[0] != ByteCode::LeaveBlock {
                            parser.byte_codes.pop_front();
                        }
                        parser.byte_codes.pop_front();
                    }
                }

                ByteCode::EnterBlock | ByteCode::LeaveBlock => {
                    //do nothing, this is only a flag
                }
            }
        }
    }
}
