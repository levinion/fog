mod func;
use std::collections::{HashMap, VecDeque};

use crate::{bytecode::ByteCode, parse::Parser, variable::Variable};

use self::func::lib_println;

pub struct VM {
    stack: VecDeque<Variable>,
    global: HashMap<String, Variable>,
}

impl VM {
    pub fn new() -> Self {
        let mut global = HashMap::new();
        global.insert("println".to_string(), Variable::Fn(lib_println));
        Self {
            stack: VecDeque::new(),
            global,
        }
    }

    pub fn execute(&mut self, parser: Parser) {
        for code in parser.byte_codes.iter() {
            match *code {
                ByteCode::GetGlobal => {
                    if let Variable::String(s) = self.stack.pop_back().unwrap() {
                        let func = self.global.get(&s).unwrap_or(&Variable::None).clone();
                        self.stack.push_back(func);
                    } else {
                        panic!("panic when get global!")
                    }
                }

                ByteCode::LoadConst { index } => {
                    self.stack.push_back(parser.constants[index].clone());
                }

                ByteCode::CallFunction { argc } => {
                    // collect args
                    let mut args = vec![];
                    for _ in 0..argc {
                        args.push(self.stack.pop_back().unwrap());
                    }
                    args.reverse();

                    // get function
                    if let Variable::Fn(func) = self.stack.pop_back().unwrap() {
                        func(args);
                    }
                }
            }
        }
    }
}
