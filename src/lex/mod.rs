use std::fs::File;
use std::io::{Read, Seek};

use crate::token::Token;

pub struct Lex {
    input: File,
}

impl From<File> for Lex {
    fn from(value: File) -> Self {
        Self { input: value }
    }
}

impl Lex {
    pub fn next(&mut self) -> Token {
        let ch = self.read_char();
        match ch {
            '\0' => Token::Eos,
            ' ' | '\r' | '\t' | '\n' => self.next(),
            '(' => Token::ParL,
            ')' => Token::ParR,
            '"' => self.read_string(),
            ch @ ('a'..='z' | 'A'..='Z' | '_') => self.read_name(ch),
            _ => todo!(),
        }
    }

    // read a name or keyword
    fn read_name(&mut self, ch: char) -> Token {
        let mut s = ch.to_string();
        loop {
            let ch = self.read_char();
            match ch {
                'a'..='z' | 'A'..='Z' | '_' => s.push(ch),
                _ => {
                    self.put_char_back();
                    break;
                }
            }
        }
        Token::Name(s)
    }

    /// read a string token from input
    fn read_string(&mut self) -> Token {
        let mut s = String::new();
        loop {
            let ch = self.read_char();
            match ch {
                '"' => break,
                '\0' => panic!("invalid string!"),
                _ => s.push(ch),
            }
        }
        Token::String(s)
    }

    /// read a char from input
    fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        if self.input.read(&mut buf).unwrap() == 1 {
            buf[0] as char
        } else {
            '\0'
        }
    }

    fn put_char_back(&mut self) {
        self.input.seek(std::io::SeekFrom::Current(-1)).unwrap();
    }
}
