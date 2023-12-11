use crate::core::{
    op::{BinaryOP, UnaryOP},
    value::Value,
};

use super::Interpreter;

macro_rules! invalid_type {
    () => {
        panic!("invalid type!")
    };
}

impl Interpreter {
    // take a stack value, push it back to stack after some operation
    pub fn unary_op(&mut self, op: UnaryOP) {
        let value = self.stack.pop_back().unwrap();
        let new_value = match op {
            // -
            UnaryOP::Sub => match value {
                Value::Float(f) => Value::Float(-f),
                Value::Int(i) => Value::Int(-i),
                _ => panic!("invalid value for sub!"),
            },
            UnaryOP::Excl => match value {
                Value::Bool(b) => Value::Bool(!b),
                _ => panic!("invalid value for excl!"),
            },
        };
        self.stack.push_back(new_value);
    }

    pub fn binary_op(&mut self, op: BinaryOP) {
        let second_value = self.stack.pop_back().unwrap();
        let fblockst_value = self.stack.pop_back().unwrap();

        let new_value = match op {
            BinaryOP::Add => match fblockst_value {
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
            BinaryOP::Sub => match fblockst_value {
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
            BinaryOP::Mul => match fblockst_value {
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
            BinaryOP::Div => match fblockst_value {
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
            BinaryOP::Equal => Value::Bool(fblockst_value == second_value),
            BinaryOP::NotEq => Value::Bool(fblockst_value != second_value),
            BinaryOP::Greater => Value::Bool(fblockst_value > second_value),
            BinaryOP::Less => Value::Bool(fblockst_value < second_value),
            BinaryOP::GreEq => Value::Bool(fblockst_value >= second_value),
            BinaryOP::LesEq => Value::Bool(fblockst_value <= second_value),
        };
        self.stack.push_back(new_value);
    }
}
