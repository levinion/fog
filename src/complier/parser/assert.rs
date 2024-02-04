use crate::core::token::{Token, TokenVal};

use super::Parser;

impl Parser {
    // next token should be that, or it will panic
    pub fn assert_next(&mut self, value: TokenVal) {
        if self.stream.next().0.val != value {
            panic!("expect token: {value:?}")
        }
    }

    pub fn assert_next_string(&mut self) -> (String, Token) {
        let token = self.stream.next();
        if let TokenVal::String(s) = token.0.val.clone() {
            (s, token)
        } else {
            panic!("expect a string!")
        }
    }

    pub fn assert_next_name(&mut self) -> (String, Token) {
        let token = self.stream.next();
        if let TokenVal::Name(s) = token.0.val.clone() {
            (s, token)
        } else {
            panic!("expect a name!")
        }
    }
}
