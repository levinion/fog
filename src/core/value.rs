use std::fmt::Display;

use crate::vm::runtime::global::Meta;

use super::block::Block;

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
    Void(()),
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
            Value::Void(v) => write!(f, "{:?}", v),
        }
    }
}

impl Value {
    pub fn typ(&self) -> Type {
        match self {
            Value::String(_) => Type::String,
            Value::Name(_) => Type::Name,
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::Function(_) => Type::Function,
            Value::Type(_) => Type::Type,
            Value::Void(_) => Type::Void,
        }
    }
}

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub enum Function {
    NormalFunction(Block), // block name
    MetaFunction(Meta),    // meta name
}

#[derive(
    PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Type {
    String,
    Name,
    Bool,
    Int,
    Float,
    Function,
    Type,
    Void,
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        match &value as &str {
            "string" => Type::String,
            "bool" => Type::Bool,
            "int" => Type::Int,
            "float" => Type::Float,
            "fn" => Type::Function,
            _ => unreachable!(),
        }
    }
}

pub trait IsType {
    fn is_type(&self) -> bool;
}

impl IsType for String {
    fn is_type(&self) -> bool {
        matches!(self as &str, "string" | "bool" | "int" | "float" | "fn")
    }
}
