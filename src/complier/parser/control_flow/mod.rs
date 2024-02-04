use crate::core::{block::Block, token::TokenVal};

use super::{wrapper, Parser};

impl Parser {
    /// enter if block
    /// [if exp {@println("hello");}]
    pub fn enter_if(&mut self, block: &mut Block) {
        self.assert_next(TokenVal::If);
        self.load_exp(block);
        wrapper::jump_if_false(block);
        self.parse_block(block);
    }

    /// parse statement in the block
    /// [{...}]
    pub fn parse_block(&mut self, block: &mut Block) {
        wrapper::enter_block(block);
        self.parse_bucket(block);
        wrapper::leave_block(block);
    }
}
