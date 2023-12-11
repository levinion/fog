use super::op::{BinaryOP, UnaryOP};

#[derive(Debug, PartialEq, Clone)]
pub enum ByteCode {
    LoadConst(usize), // index of constants table
    GetGlobal,
    CallMetaFunction(usize), // the number of args that function takes
    CallFunction(usize),
    CallFogFunction(usize),
    StoreLocal(usize), // index of locals list
    LoadLocal(usize),  // index of locals list
    JumpIfFalse,
    EnterBlock,
    LeaveBlock,
    UnaryOP(UnaryOP),
    BinaryOP(BinaryOP),
}
