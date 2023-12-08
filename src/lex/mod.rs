mod util;
use std::{collections::VecDeque, fs::File};

use crate::token::Token;

pub struct Lex {
    input: File,
    pre_buf: VecDeque<Token>,
}

impl From<File> for Lex {
    fn from(value: File) -> Self {
        Self {
            input: value,
            pre_buf: VecDeque::new(),
        }
    }
}

impl Lex {
    pub fn next(&mut self) -> Token {
        // if there is some value in pre_buf, then return it.
        if !self.pre_buf.is_empty() {
            let token = self.pre_buf.pop_front().unwrap();
            return token;
        }
        let ch = self.read_char();
        match ch {
            '\0' => Token::Eos,
            ' ' | '\r' | '\t' | '\n' => self.next(),
            '(' => Token::ParL,
            ')' => Token::ParR,
            '=' => self.check_next_char('=', Token::Equal, Token::Assign),
            '"' => self.read_string(),
            ',' => Token::Comma,
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
        // parse keyword
        match &s as &str {
            "let" => Token::Let,
            _ => Token::Name(s),
        }
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
}
