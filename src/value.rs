use std::fmt::Display;

pub type Args = Vec<Value>;

#[derive(Clone)]
pub enum Value {
    String(String),
    Fn(fn(Args) -> i32),
    Name(String),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::String(s) => write!(f, "{}", s),
            Value::Fn(func) => write!(f, "{:?}", func),
            Value::None => write!(f, "none"),
            Value::Name(name) => write!(f, "name: {}", name),
        }
    }
}
