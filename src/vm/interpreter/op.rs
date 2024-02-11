use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::core::{
    op::{BinaryOP, UnaryOP},
    value::Value,
};

use super::Interpreter;

macro_rules! invalid_type {
    () => {
        return Err(anyhow!("invalid type!"))
    };
}

impl Interpreter {
    // take a stack value, push it back to stack after some operation
    pub fn unary_op(&mut self, op: UnaryOP) -> Result<()> {
        let value = self.stack.pop_back().unwrap();
        let new_value = match op {
            // -
            UnaryOP::Sub => match value {
                Value::Float(f) => Value::Float(-f),
                Value::Int(i) => Value::Int(-i),
                _ => return Err(anyhow!("invalid value for sub!")),
            },
            UnaryOP::Excl => match value {
                Value::Bool(b) => Value::Bool(!b),
                _ => return Err(anyhow!("invalid value for excl!")),
            },
        };
        self.stack.push_back(new_value);
        Ok(())
    }

    pub fn binary_op(&mut self, op: BinaryOP) -> Result<()> {
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
                    Value::String(s2) => Value::String(Arc::new(s1.to_string() + s2.as_str())),
                    _ => invalid_type!(),
                },
                Value::Type(t1) => match second_value {
                    Value::Type(t2) => {
                        if t1 == t2 {
                            Value::Type(t1)
                        } else {
                            invalid_type!()
                        }
                    }
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
                Value::Type(t1) => match second_value {
                    Value::Type(t2) => {
                        if t1 == t2 {
                            Value::Type(t1)
                        } else {
                            invalid_type!()
                        }
                    }
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
                Value::Type(t1) => match second_value {
                    Value::Type(t2) => {
                        if t1 == t2 {
                            Value::Type(t1)
                        } else {
                            invalid_type!()
                        }
                    }
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
                Value::Type(t1) => match second_value {
                    Value::Type(t2) => {
                        if t1 == t2 {
                            Value::Type(t1)
                        } else {
                            invalid_type!()
                        }
                    }
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
        Ok(())
    }
}
