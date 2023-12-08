use super::op::{BinaryOP, UnaryOP};

#[derive(Debug, PartialEq)]
pub enum ByteCode {
    LoadConst { index: usize }, // index of constants table
    GetGlobal,
    CallFunction { argc: usize }, // the number of args that function takes
    StoreLocal { index: usize },  // index of locals list
    LoadLocal { index: usize },   // index of locals list
    JumpIfFalse,
    EnterBlock,
    LeaveBlock,
    UnaryOP(UnaryOP),
    BinaryOP(BinaryOP),
}
