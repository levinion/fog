pub mod token_stream;
mod util;
use std::fs::File;

use crate::core::token::Token;

use self::token_stream::TokenStream;

pub struct Lex {
    input: File,
    pre_read_token: Option<Token>,
}

impl From<File> for Lex {
    fn from(value: File) -> Self {
        Self {
            input: value,
            pre_read_token: None,
        }
    }
}

impl Lex {
    pub fn into_token_stream(mut self) -> TokenStream {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let token = self.next();
            tokens.push(token);
            if tokens.last() == Some(&Token::Eos) {
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
        let ch = self.read_char();
        match ch {
            '\0' => Token::Eos,
            ' ' | '\r' | '\t' | '\n' => self.do_next(),
            '(' => Token::ParL,
            ')' => Token::ParR,
            '=' => self.check_next_char('=', Token::Equal, Token::Assign),
            '!' => self.check_next_char('=', Token::NotEq, Token::Excl),
            '>' => self.check_next_char('=', Token::GreEq, Token::Greater),
            '<' => self.check_next_char('=', Token::LesEq, Token::Less),
            '"' => self.read_string(),
            ',' => Token::Comma,
            '{' => Token::CurlyL,
            '}' => Token::CurlyR,
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,
            ';' => Token::SemiColon,
            ch @ ('0'..='9' | '.') => self.read_number(ch),
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
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "if" => Token::If,
            "else" => Token::Else,
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

    /// read number
    fn read_number(&mut self, ch: char) -> Token {
        let mut s = ch.to_string();
        loop {
            let ch = self.read_char();
            match ch {
                '0'..='9' | '.' | '_' => s.push(ch),
                _ => {
                    self.put_char_back();
                    break;
                }
            }
        }
        if s.contains('.') {
            let n = s.parse::<f64>().expect("invalid number!");
            Token::Float(n)
        } else {
            let n = s.parse::<i64>().expect("invalid number!");
            Token::Int(n)
        }
    }
}
