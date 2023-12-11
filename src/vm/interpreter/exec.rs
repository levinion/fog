use crate::core::value::{Args, Value};

use super::Interpreter;

impl Interpreter {
    // take a function name constant and args, call the function.
    pub fn call_meta_function(&mut self, argc: usize) {
        let args = self.collect_args(argc);
        // get function
        if let Value::Fn(func) = self.stack.pop_back().unwrap() {
            func(args);
        }
    }

    // take a function name constant and args, call the function.
    // argc: args number that to be load
    pub async fn call_fog_function(&mut self, argc: usize) {
        let args = self.collect_args(argc);
        if let Value::String(name) = self.stack.pop_back().unwrap() {
            self.manager.par_exec(&name, args).await;
        } else {
            panic!("invalid function name type");
        }
    }

    pub async fn call_function(&mut self, argc: usize) {
        let args = self.collect_args(argc);
        if let Value::String(name) = self.stack.pop_back().unwrap() {
            self.manager.exec(&name, args).await;
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
