use std::{fmt::Display, sync::Arc};

use crate::vm::runtime::global::Meta;

use super::{
    block::Block,
    token::{Token, TokenVal},
};

pub type Args = Vec<Value>;

#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub enum Value {
    String(Arc<String>),
    Name(Arc<String>),
    Type(Type),
    Bool(bool),
    Int(i64),
    Float(f64),
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

impl From<Token> for Value {
    fn from(value: Token) -> Self {
        match value.0.val.clone() {
            TokenVal::Name(v) => Value::Name(v),
            TokenVal::String(v) => Value::String(v),
            TokenVal::Bool(v) => Value::Bool(v),
            TokenVal::Int(v) => Value::Int(v),
            TokenVal::Float(v) => Value::Float(v),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum Function {
    NormalFunction(Arc<Block>), // a normal func is a block
    #[serde(skip)]
    MetaFunction(Meta), // meta name -> meta func
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
