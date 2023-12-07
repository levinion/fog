use std::fmt::Display;

pub type Args = Vec<Variable>;

#[derive(Clone)]
pub enum Variable {
    String(String),
    Fn(fn(Args) -> i32),
    None,
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Variable::String(s) => write!(f, "{}", s),
            Variable::Fn(func) => write!(f, "{:?}", func),
            Variable::None => write!(f, "none"),
        }
    }
}
