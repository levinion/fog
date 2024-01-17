use std::fmt::Display;

pub type Args = Vec<Value>;

#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub enum Value {
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Function(Function),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(n) => write!(f, "{:#?}", n),
            Value::None => write!(f, "none"),
            Value::Function(func) => write!(f, "to be completed!"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum Function {
    NormalFunction = 0,
    MetaFunction,
}
