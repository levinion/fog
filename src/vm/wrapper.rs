use crate::{
    core::{
        op::{BinaryOP, UnaryOP},
        value::Value,
    },
    parse::ir::IR,
};

use super::VM;

macro_rules! invalid_type {
    () => {
        panic!("invalid type!")
    };
}

impl VM {
    /// take a element then get global variable, usually a function
    pub fn get_global(&mut self) {
        if let Value::String(s) = self.stack.pop_back().unwrap() {
            let func = self.global_table.get(&s).unwrap_or(&Value::None).clone();
            self.stack.push_back(func);
        } else {
            panic!("panic when get global!")
        }
    }

    //  load a value to the stack
    pub fn load_const(&mut self, ir: &mut IR, index: usize) {
        self.stack.push_back(ir.constants[index].clone());
    }

    // take a function name constant and args, call the function.
    pub fn call_function(&mut self, argc: usize) {
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
    pub fn store_local(&mut self, ir: &mut IR, index: usize) {
        let value = self.stack.pop_back().unwrap();
        let name = ir.locals.get(index).unwrap().clone();
        self.local_table.insert(name, value);
    }

    // take a name, and load the value.
    pub fn load_local(&mut self, ir: &mut IR, index: usize) {
        let name = ir.locals.get(index).unwrap().clone();
        self.stack
            .push_back(self.local_table.get(&name).unwrap().clone());
    }

    pub fn jump_if_false(&mut self, ir: &mut IR) {
        let b = if let Value::Bool(b) = self.stack.pop_back().unwrap() {
            b
        } else {
            panic!("expected bool!")
        };
        if !b {
            ir.jump_block();
        }
    }

    // take a stack value, push it back to stack after some operation
    pub fn unary_op(&mut self, op: UnaryOP) {
        let value = self.stack.pop_back().unwrap();
        let new_value = match op {
            // -
            UnaryOP::Sub => match value {
                Value::Float(f) => Value::Float(-f),
                Value::Int(i) => Value::Int(-i),
                _ => panic!("invalid value for that op!"),
            },
        };
        self.stack.push_back(new_value);
    }

    pub fn binary_op(&mut self, op: BinaryOP) {
        let second_value = self.stack.pop_back().unwrap();
        let first_value = self.stack.pop_back().unwrap();

        let new_value = match op {
            BinaryOP::Add => match first_value {
                Value::Float(f1) => match second_value {
                    Value::Float(f2) => Value::Float(f1 + f2),
                    _ => invalid_type!(),
                },
                Value::Int(i1) => match second_value {
                    Value::Int(i2) => Value::Int(i1 + i2),
                    _ => invalid_type!(),
                },
                Value::String(s1) => match second_value {
                    Value::String(s2) => Value::String(s1 + s2.as_str()),
                    _ => invalid_type!(),
                },
                _ => invalid_type!(),
            },
            BinaryOP::Sub => match first_value {
                Value::Float(f1) => match second_value {
                    Value::Float(f2) => Value::Float(f1 - f2),
                    _ => invalid_type!(),
                },
                Value::Int(i1) => match second_value {
                    Value::Int(i2) => Value::Int(i1 - i2),
                    _ => invalid_type!(),
                },
                _ => invalid_type!(),
            },
            BinaryOP::Mul => match first_value {
                Value::Float(f1) => match second_value {
                    Value::Float(f2) => Value::Float(f1 * f2),
                    _ => invalid_type!(),
                },
                Value::Int(i1) => match second_value {
                    Value::Int(i2) => Value::Int(i1 * i2),
                    _ => invalid_type!(),
                },
                _ => invalid_type!(),
            },
            BinaryOP::Div => match first_value {
                Value::Float(f1) => match second_value {
                    Value::Float(f2) => Value::Float(f1 / f2),
                    _ => invalid_type!(),
                },
                Value::Int(i1) => match second_value {
                    Value::Int(i2) => Value::Int(i1 / i2),
                    _ => invalid_type!(),
                },
                _ => invalid_type!(),
            },
            BinaryOP::Equal => Value::Bool(first_value == second_value),
            BinaryOP::NotEq => Value::Bool(first_value != second_value),
            BinaryOP::Greater => Value::Bool(first_value > second_value),
            BinaryOP::Less => Value::Bool(first_value < second_value),
            BinaryOP::GreEq => Value::Bool(first_value >= second_value),
            BinaryOP::LesEq => Value::Bool(first_value <= second_value),
        };
        self.stack.push_back(new_value);
    }
}
