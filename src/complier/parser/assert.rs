use crate::core::token::Token;

use super::Parser;

impl Parser {
    // next token should be that, or it will panic
    pub fn assert_next(&mut self, token: Token) {
        if self.stream.next() != token {
            panic!("expect token: {token:?}")
        }
    }

    pub fn assert_next_string(&mut self) -> String {
        if let Token::String(s) = self.stream.next() {
            s
        } else {
            panic!("expect a string!")
        }
    }

    pub fn assert_next_name(&mut self) -> String {
        if let Token::Name(s) = self.stream.next() {
            s
        } else {
            panic!("expect a name!")
        }
    }
}
