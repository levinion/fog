use std::fmt::Display;

pub type Args = Vec<Value>;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    String(String),
    Fn(fn(Args) -> i32),
    Bool(bool),
    Int(i64),
    Float(f64),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::String(s) => write!(f, "{}", s),
            Value::Fn(func) => write!(f, "{:?}", func),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(n) => write!(f, "{:#?}", n),
            Value::None => write!(f, "none"),
        }
    }
}
