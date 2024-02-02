mod coliner;
pub mod token_stream;
use std::io::prelude::*;

use crate::core::token::{Token, TokenVal};

use self::{coliner::Coliner, token_stream::TokenStream};

pub struct Lexer<T: Read + Seek> {
    pre_read_token: Option<Token>,
    coliner: Coliner<T>,
}

impl<T: Read + Seek> From<T> for Lexer<T> {
    fn from(value: T) -> Self {
        Self {
            coliner: Coliner::new(value),
            pre_read_token: None,
        }
    }
}

impl<T: Read + Seek> Lexer<T> {
    pub fn into_token_stream(mut self) -> TokenStream {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let token = self.next();
            tokens.push(token);
            if tokens.last().unwrap().val == TokenVal::Eos {
                break;
            }
        }
        TokenStream { pc: 0, tokens }
    }

    fn next(&mut self) -> Token {
        // if there is some value in pre_buf, then return it.
        if self.pre_read_token.is_some() {
            let token = self.pre_read_token.take().unwrap();
            return token;
        }
        self.do_next()
    }

    fn do_next(&mut self) -> Token {
        let start = self.coliner.current();
        let ch = self.coliner.read_char();
        let token;
        match ch {
            '\0' => token = TokenVal::Eos,
            ' ' | '\r' | '\t' | '\n' => return self.do_next(),
            '(' => token = TokenVal::ParL,
            ')' => token = TokenVal::ParR,
            '=' => {
                token = self
                    .coliner
                    .check_next_char('=', TokenVal::Equal, TokenVal::Assign)
            }
            '!' => {
                token = self
                    .coliner
                    .check_next_char('=', TokenVal::NotEq, TokenVal::Excl)
            }
            '>' => {
                token = self
                    .coliner
                    .check_next_char('=', TokenVal::GreEq, TokenVal::Greater)
            }
            '<' => {
                token = self
                    .coliner
                    .check_next_char('=', TokenVal::LesEq, TokenVal::Less)
            }
            '"' => token = self.read_string(),
            ',' => token = TokenVal::Comma,
            '{' => token = TokenVal::CurlyL,
            '}' => token = TokenVal::CurlyR,
            '+' => token = TokenVal::Add,
            '-' => token = TokenVal::Sub,
            '*' => token = TokenVal::Mul,
            '/' => token = TokenVal::Div,
            ';' => token = TokenVal::SemiColon,
            '.' => token = TokenVal::Dot,
            ':' => token = TokenVal::Colon,
            ch @ ('0'..='9') => token = self.read_number(ch),
            ch @ ('@' | 'a'..='z' | 'A'..='Z' | '_') => token = self.read_name(ch),
            _ => todo!(),
        };
        let end = self.coliner.current();
        Token::new(token, start, end)
    }

    // read a name or keyword
    fn read_name(&mut self, ch: char) -> TokenVal {
        let mut s = ch.to_string();
        loop {
            let ch = self.coliner.read_char();
            match ch {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => s.push(ch),
                ':' => {
                    let next = self.coliner.read_char();
                    if next == ':' {
                        s.push_str("::");
                    } else {
                        self.coliner.put_char_back();
                        self.coliner.put_char_back();
                        break;
                    }
                }
                _ => {
                    self.coliner.put_char_back();
                    break;
                }
            }
        }
        // parse keyword
        match &s as &str {
            "let" => TokenVal::Let,
            "true" => TokenVal::Bool(true),
            "false" => TokenVal::Bool(false),
            "if" => TokenVal::If,
            "else" => TokenVal::Else,
            "fn" => TokenVal::Fn,
            "fog" => TokenVal::Fog,
            _ => TokenVal::Name(s),
        }
    }

    /// read a string token from input
    fn read_string(&mut self) -> TokenVal {
        let mut s = String::new();
        loop {
            let ch = self.coliner.read_char();
            match ch {
                '"' => break,
                '\0' => panic!("invalid string!"),
                '\\' => {
                    let ch = self.coliner.read_char();
                    match ch {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        _ => self.coliner.put_char_back(),
                    }
                }
                _ => s.push(ch),
            }
        }
        TokenVal::String(s)
    }

    /// read number
    fn read_number(&mut self, ch: char) -> TokenVal {
        let mut s = ch.to_string();
        loop {
            let ch = self.coliner.read_char();
            match ch {
                '0'..='9' | '.' | '_' => s.push(ch),
                _ => {
                    self.coliner.put_char_back();
                    break;
                }
            }
        }
        if s.contains('.') {
            let n = s.parse::<f64>().expect("invalid number!");
            TokenVal::Float(n)
        } else {
            let n = s.parse::<i64>().expect("invalid number!");
            TokenVal::Int(n)
        }
    }
}
