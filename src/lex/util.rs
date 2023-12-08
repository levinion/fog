use crate::token::Token;

use super::Lex;
use std::io::{Read, Seek};

impl Lex {
    /// read a char from input
    pub fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        if self.input.read(&mut buf).unwrap() == 1 {
            buf[0] as char
        } else {
            '\0'
        }
    }

    /// put a char back
    pub fn put_char_back(&mut self) {
        self.input.seek(std::io::SeekFrom::Current(-1)).unwrap();
    }

    // check the next char, if it matches "this", then return it, or return the other token.
    pub fn check_next_char(&mut self, ch: char, this: Token, other: Token) -> Token {
        if self.read_char() == ch {
            this
        } else {
            self.put_char_back();
            other
        }
    }

    // pre-read some token, and then put them in the pre_buf.
    fn pre_read_token(&mut self, num: usize) {
        for _ in 0..num {
            let token = self.next();
            self.pre_buf.push_back(token);
        }
    }

    // look ahead token
    pub fn look_ahead(&mut self, num: usize) -> &Token {
        let num = num - self.pre_buf.len();
        self.pre_read_token(num);
        self.pre_buf.get(num - 1).unwrap()
    }
}
