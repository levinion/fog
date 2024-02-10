// use crate::core::{
//     block::Block,
//     bytecode::{ByteCode, Decorate},
//     token::TokenVal,
// };
//
// use super::Parser;
//
// impl Parser {
//     /// enter if block
//     /// [if exp {@println("hello");}]
//     pub fn enter_if(&mut self, block: &mut Block) {
//         self.assert_next(TokenVal::If);
//         self.load_exp(block);
//         block.byte_codes.push(ByteCode::JumpIfFalse);
//         self.parse_block(block);
//     }
//
//     /// parse statement in the block
//     /// [{...}]
//     pub fn parse_block(&mut self, block: &mut Block) {
//         block
//             .byte_codes
//             .push(ByteCode::Decorate(Decorate::EnterBlock));
//         self.parse_bucket(block);
//         block
//             .byte_codes
//             .push(ByteCode::Decorate(Decorate::LeaveBlock));
//     }
// }
