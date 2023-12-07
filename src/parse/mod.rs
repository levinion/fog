mod assert;
use crate::{bytecode::ByteCode, lex::Lex, token::Token, variable::Variable};

pub struct Parser {
    lex: Lex,
    pub byte_codes: Vec<ByteCode>,
    pub constants: Vec<Variable>,
}

impl From<Lex> for Parser {
    fn from(value: Lex) -> Self {
        let byte_codes = vec![];
        let constants = vec![];
        Self {
            lex: value,
            byte_codes,
            constants,
        }
    }
}

impl Parser {
    pub fn parse(&mut self) {
        loop {
            let token = self.lex.next();
            match token {
                Token::Name(name) => self.call_function(name),
                Token::Eos => break,
                _ => todo!(),
            }
        }
    }

    /// call a function whose name is name
    fn call_function(&mut self, name: String) {
        // add function name to constants table
        self.constants.push(Variable::String(name));
        // load function name to the stack
        self.byte_codes.push(ByteCode::LoadConst {
            index: self.constants.len() - 1,
        });
        // get the function from global and load to the stack
        self.byte_codes.push(ByteCode::GetGlobal);

        self.assert_next(Token::ParL);
        if let Token::String(s) = self.lex.next() {
            // add argument name to constants table and then load it to stack
            self.constants.push(Variable::String(s));
            self.byte_codes.push(ByteCode::LoadConst {
                index: self.constants.len() - 1,
            });
        } else {
            panic!("expected string!");
        }
        self.assert_next(Token::ParR);

        // call function
        self.byte_codes.push(ByteCode::CallFunction { argc: 1 });
    }
}
