use std::fmt::Display;

use crate::vm::runtime::global::Meta;

use super::{block::Block, typ::Type};

pub type Args = Vec<Value>;

#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub enum Value {
    String(String),
    Name(String),
    Type(Type),
    Bool(bool),
    Int(i64),
    Float(f64),
    #[serde(skip)]
    Function(Function),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::String(s) => write!(f, "{}", s),
            Value::Name(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(n) => write!(f, "{:#?}", n),
            Value::Function(func) => match func {
                Function::NormalFunction(name) => write!(f, "{:?}", name),
                Function::MetaFunction(name) => write!(f, "{:?}", name),
            },
            Value::Type(t) => write!(f, "{:?}", t),
        }
    }
}

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub enum Function {
    NormalFunction(Block), // block name
    MetaFunction(Meta),    // meta name
}
