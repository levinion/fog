use std::collections::HashMap;

use crate::core::{
    block::Block,
    bytecode::{ByteCode, Decorate, FunctionType},
    ir::IR,
};

pub fn optimize(ir: IR) -> HashMap<String, Block> {
    let mut blocks: HashMap<String, Block> = ir.into();
    blocks.iter_mut().for_each(|(_, block)| {
        while let Some(code) = block.go_ahead() {
            match code {
                ByteCode::Decorate(decorate) => {
                    let decorate = *decorate;
                    let decorate: Decorate = decorate.into();
                    match decorate {
                        Decorate::Fog => {
                            // when find fog, remove it, then modify the call bytecode.
                            block.pc -= 1;
                            block.byte_codes.remove(block.pc);
                            let index = block
                                .byte_codes
                                .iter()
                                .enumerate()
                                .filter(|(index, _)| *index >= block.pc)
                                .filter(|(_, c)| matches!(c, ByteCode::CallFunction(_, _)))
                                .map(|(index, _)| index)
                                .next()
                                .unwrap();
                            let c = block.byte_codes.get_mut(index).unwrap();
                            if let ByteCode::CallFunction(_, t) = c {
                                *t = FunctionType::FogFunction as usize;
                            }
                            block.reset_pc();
                        }
                        Decorate::EnterBlock => {
                            todo!();
                        }
                        Decorate::LeaveBlock => {
                            todo!();
                        }
                    }
                }
                ByteCode::CallFunction(_, _) => {
                    let c = block.byte_codes.get_mut(block.pc - 1).unwrap();
                    if let ByteCode::CallFunction(_, t) = c {
                        *t = FunctionType::NormalFunction as usize;
                    }
                    block.reset_pc();
                }
                _ => {}
            }
        }
    });
    blocks
}
