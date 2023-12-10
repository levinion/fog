use crate::core::token::Token;

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
}
