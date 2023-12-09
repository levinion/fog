use crate::core::token::Token;

use super::Parser;

impl Parser {
    /// enter if block
    /// eg: ```
    /// if true{
    ///     println("hello")
    /// }```
    pub fn enter_if(&mut self) {
        self.load_exp();
        self.jump_if_false();
        self.parse_block();
    }

    /// parse statement in the block
    pub fn parse_block(&mut self) {
        self.assert_next(Token::CurlyL);
        self.enter_block();
        loop {
            if let Some(Token::CurlyR) = self.parse_once() {
                break;
            }
        }
        self.leave_block();
    }
}
