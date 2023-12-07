use crate::token::Token;

use super::Parser;

impl Parser {
    pub fn assert_next(&mut self, token: Token) {
        if self.lex.next() != token {
            panic!("expect token: {token:?}")
        }
    }
}
