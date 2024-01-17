use super::op::{BinaryOP, UnaryOP};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ByteCode {
    LoadConst(usize), // index of constants table
    GetGlobal,
    Decorate(usize),
    CallFunction(usize, usize), // argc,isFogFunction
    CallMethod(usize),
    StoreLocal(usize), // index of locals list
    LoadLocal(usize),  // index of locals list
    JumpIfFalse,
    UnaryOP(UnaryOP),
    BinaryOP(BinaryOP),
}

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
