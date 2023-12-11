use crate::core::value::Value;

use super::Interpreter;

impl<'a> Interpreter<'a> {
    // take a function name constant and args, call the function.
    pub fn call_meta_function(&mut self, argc: usize) {
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

    // take a function name constant and args, call the function.
    pub async fn call_fog_function(&mut self, argc: usize) {
        // collect args
        let mut args = vec![];
        for _ in 0..argc {
            args.push(self.stack.pop_back().unwrap());
        }
        args.reverse();
        if let Value::String(name) = self.stack.pop_back().unwrap() {
            let block = self.block_table.get(&name).unwrap().clone();
            let block_table = self.block_table.clone();
            tokio::spawn(async move {
                let mut new_interpreter = Interpreter::new(&block_table);
                new_interpreter.execute(block).await;
            });
        } else {
            panic!("invalid function name type");
        }
    }

    pub async fn call_function(&mut self, argc: usize) {
        // collect args
        let mut args = vec![];
        for _ in 0..argc {
            args.push(self.stack.pop_back().unwrap());
        }
        args.reverse();
        if let Value::String(name) = self.stack.pop_back().unwrap() {
            let mut new_interpreter = Interpreter::new(self.block_table);
            let block = self.block_table.get(&name).unwrap();
            new_interpreter.execute(block.clone()).await;
        } else {
            panic!("invalid function name type");
        }
    }
}
