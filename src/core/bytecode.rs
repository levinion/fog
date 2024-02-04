use super::{
    op::{BinaryOP, UnaryOP},
    value::Value,
};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, PartialOrd)]
pub enum ByteCode {
    LoadName,
    Decorate(Decorate),
    CallFunction(usize), // argc,isFogFunction
    FogCallFunction(usize),
    CallMethod(usize),
    LoadValue(Value),
    StoreLocal,
    JumpIfFalse,
    UnaryOP(UnaryOP),
    BinaryOP(BinaryOP),
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum Decorate {
    EnterBlock,
    LeaveBlock,
}

impl From<usize> for Decorate {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::EnterBlock,
            1 => Self::LeaveBlock,
            _ => unreachable!(),
        }
    }
}
