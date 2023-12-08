use crate::token::Token;

use super::Parser;

impl Parser {
    // next token should be that, or it will panic
    pub fn assert_next(&mut self, token: Token) {
        if self.lex.next() != token {
            panic!("expect token: {token:?}")
        }
    }
}
