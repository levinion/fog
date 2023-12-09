use super::op::{BinaryOP, UnaryOP};

#[derive(Debug, PartialEq)]
pub enum ByteCode {
    LoadConst(usize), // index of constants table
    GetGlobal,
    CallFunction(usize), // the number of args that function takes
    StoreLocal(usize),   // index of locals list
    LoadLocal(usize),    // index of locals list
    JumpIfFalse,
    EnterBlock,
    LeaveBlock,
    UnaryOP(UnaryOP),
    BinaryOP(BinaryOP),
}
