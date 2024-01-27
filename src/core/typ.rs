use super::value::Value;

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
}

impl From<Value> for Type {
    fn from(value: Value) -> Self {
        match value {
            Value::String(_) => Self::String,
            Value::Name(_) => Self::Name,
            Value::Bool(_) => Self::Bool,
            Value::Int(_) => Self::Int,
            Value::Float(_) => Self::Float,
            Value::Function(_) => Self::Function,
            Value::Type(_) => Self::Type,
        }
    }
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
