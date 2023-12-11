use crate::core::token::Token;

#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
    pub pc: usize, // point to the next token to be read
}

impl TokenStream {
    pub fn next(&mut self) -> Token {
        let token = self.tokens.get(self.pc).unwrap().clone();
        self.pc += 1;
        token
    }

    pub fn look_ahead(&self, ahead: usize) -> &Token {
        self.tokens.get(self.pc + ahead - 1).unwrap()
    }
}
