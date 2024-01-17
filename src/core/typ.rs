use super::value::Value;

#[derive(PartialEq, Eq, Hash)]
pub enum Type {
    String,
    Bool,
    Int,
    Float,
    Function,
    None,
}

impl From<Value> for Type {
    fn from(value: Value) -> Self {
        match value {
            Value::String(_) => Self::String,
            Value::Bool(_) => Self::Bool,
            Value::Int(_) => Self::Int,
            Value::Float(_) => Self::Float,
            Value::None => Self::None,
            Value::Function(_) => Self::Function,
        }
    }
}
