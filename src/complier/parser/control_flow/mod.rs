use crate::core::{block::Block, token::Token};

use super::Parser;

impl Parser {
    /// enter if block
    /// eg: ```
    /// if true{
    ///     println("hello")
    /// }```
    pub fn enter_if(&mut self, block: &mut Block) {
        self.assert_next(Token::If);
        self.load_exp(block);
        self.jump_if_false(block);
        self.parse_block(block);
    }

    /// parse statement in the block
    pub fn parse_block(&mut self, block: &mut Block) {
        self.enter_block(block);
        self.parse_curly_pair(block);
        self.leave_block(block);
    }
}
