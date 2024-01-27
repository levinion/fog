use crate::core::{
    block::Block,
    bytecode::{ByteCode, Decorate, FunctionType},
    value::Value,
};

pub fn load_value(block: &mut Block, value: Value) {
    block.byte_codes.push(ByteCode::LoadValue(value));
}

pub fn load_name(block: &mut Block) {
    block.byte_codes.push(ByteCode::LoadName);
}

pub fn decorate(block: &mut Block, decorate: Decorate) {
    block.byte_codes.push(ByteCode::Decorate(decorate))
}

// take the function and args then call it.
pub fn call_function(block: &mut Block, argc: usize, function_type: FunctionType) {
    block
        .byte_codes
        .push(ByteCode::CallFunction(argc, function_type));
}

pub fn call_method(block: &mut Block, argc: usize) {
    block.byte_codes.push(ByteCode::CallMethod(argc));
}

// store the name to locals and return its index
pub fn store_local(block: &mut Block) {
    block.byte_codes.push(ByteCode::StoreLocal);
}

pub fn jump_if_false(block: &mut Block) {
    block.byte_codes.push(ByteCode::JumpIfFalse);
}

pub fn enter_block(block: &mut Block) {
    block
        .byte_codes
        .push(ByteCode::Decorate(Decorate::EnterBlock));
}

pub fn leave_block(block: &mut Block) {
    block
        .byte_codes
        .push(ByteCode::Decorate(Decorate::LeaveBlock));
}
