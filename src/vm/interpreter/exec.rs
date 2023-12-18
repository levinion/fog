use crate::core::{
    block::Block,
    value::{Args, MetaFunc, Value},
};

use super::{
    meta::{lib_exit, lib_print, lib_println},
    Interpreter,
};

impl Interpreter {
    // take a function name constant and args, call the function.
    pub fn call_meta_function(&mut self, argc: usize) {
        let args = self.collect_args(argc);
        // get function
        if let Value::MetaFunc(func) = self.stack.pop_back().unwrap() {
            match func {
                MetaFunc::Println => lib_println(args),
                MetaFunc::Print => lib_print(args),
                MetaFunc::Exit => lib_exit(args),
            };
        }
    }

    // take a function name constant and args, call the function.
    // argc: args number that to be load
    pub async fn call_fog_function(&mut self, block: &Block, argc: usize) {
        let args = self.collect_args(argc);
        if let Value::String(name) = self.stack.pop_back().unwrap() {
            self.manager.par_exec(&name, args, block.namespace()).await;
        } else {
            panic!("invalid function name type");
        }
    }

    pub async fn call_function(&mut self, block: &Block, argc: usize) {
        let args = self.collect_args(argc);
        if let Value::String(name) = self.stack.pop_back().unwrap() {
            self.manager.exec(&name, args, block.namespace()).await;
        } else {
            panic!("invalid function name type");
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
}
