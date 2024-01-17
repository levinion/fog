use crate::core::{
    block::Block,
    bytecode::{ByteCode, Decorate, FunctionType},
    value::Value,
};

/// add argument name to constants table and then load it to stack
pub fn load_const(block: &mut Block, value: Value) {
    // if there exists the value, then just return it.
    if let Some(index) = block.constants.iter().position(|x| x == &value) {
        block.byte_codes.push(ByteCode::LoadConst(index));
        return;
    }
    block.constants.push(value);
    block
        .byte_codes
        .push(ByteCode::LoadConst(block.constants.len() - 1));
}

// get from global and load to the stack
pub fn get_global(block: &mut Block) {
    block.byte_codes.push(ByteCode::GetGlobal);
}

pub fn decorate(block: &mut Block, decorate: Decorate) {
    block.byte_codes.push(ByteCode::Decorate(decorate as usize))
}

// take the function and args then call it.
pub fn call_function(block: &mut Block, argc: usize, function_type: FunctionType) {
    block
        .byte_codes
        .push(ByteCode::CallFunction(argc, function_type as usize));
}

pub fn call_method(block: &mut Block, argc: usize) {
    block.byte_codes.push(ByteCode::CallMethod(argc));
}

// store the name to locals and return its index
pub fn store_local(block: &mut Block, name: String) {
    // if there exists the value, then just return it.
    if let Some(index) = block.locals.iter().position(|x| x == &name) {
        block.byte_codes.push(ByteCode::StoreLocal(index));
        return;
    }

    block.locals.push(name);
    block
        .byte_codes
        .push(ByteCode::StoreLocal(block.locals.len() - 1));
}

// load local variable from the locals
pub fn load_local(block: &mut Block, name: String) {
    let index = block
        .locals
        .iter()
        .rposition(|x| *x == name)
        .unwrap_or_else(|| panic!("name not found: {name}"));
    block.byte_codes.push(ByteCode::LoadLocal(index));
}

pub fn jump_if_false(block: &mut Block) {
    block.byte_codes.push(ByteCode::JumpIfFalse);
}

pub fn enter_block(block: &mut Block) {
    block
        .byte_codes
        .push(ByteCode::Decorate(Decorate::EnterBlock as usize));
}

pub fn leave_block(block: &mut Block) {
    block
        .byte_codes
        .push(ByteCode::Decorate(Decorate::LeaveBlock as usize));
}
