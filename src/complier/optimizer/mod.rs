use std::collections::HashMap;

use crate::core::{
    block::Block,
    bytecode::{ByteCode, Decorate},
    ir::{IR1, IR2},
};

pub fn optimize(ir: IR1) -> IR2 {
    let mut blocks: HashMap<String, Block> = ir.into();
    blocks.iter_mut().for_each(|(_, block)| {
        while let Some(code) = block.go_ahead() {
            match code {
                ByteCode::Decorate(decorate) => match *decorate {
                    Decorate::EnterBlock => {
                        todo!();
                    }
                    Decorate::LeaveBlock => {
                        todo!();
                    }
                },
                _ => {}
            }
        }
    });
    blocks.iter_mut().for_each(|(_, block)| block.reset_pc());
    IR2(blocks)
}
