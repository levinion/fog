use super::{
    op::{BinaryOP, UnaryOP},
    value::Value,
};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, PartialOrd)]
pub enum ByteCode {
    LoadName,
    Decorate(Decorate),
    CallFunction(usize, FunctionType), // argc,isFogFunction
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
    Fog = 0,
    EnterBlock,
    LeaveBlock,
}

impl From<usize> for Decorate {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Fog,
            1 => Self::EnterBlock,
            2 => Self::LeaveBlock,
            _ => unreachable!(),
        }
    }
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum FunctionType {
    NormalFunction = 0,
    FogFunction,
    Undefined,
}

impl From<usize> for FunctionType {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::FogFunction,
            1 => Self::NormalFunction,
            2 => Self::Undefined,
            _ => unreachable!(),
        }
    }
}
