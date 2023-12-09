use super::token::Token;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum UnaryOP {
    Sub = 0, // -
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BinaryOP {
    Add = 0, // +
    Sub,     // -
    Mul,     // *
    Div,     // /
    Equal,   // ==
    NotEq,   // !=
    Greater, // >
    Less,    // <
    GreEq,   // >=
    LesEq,   // <=
}

/// only used in parser to get final expression
#[derive(PartialEq, Debug)]
pub enum InfixBinaryOP {
    Add = 0, // +
    Sub,     // -
    Mul,     // *
    Div,     // /
    ParL,    // (
    ParR,    // )
    Equal,   // ==
    NotEq,   // !=
    Greater, // >
    Less,    // <
    GreEq,   // >=
    LesEq,   // <=
}

impl InfixBinaryOP {
    pub fn priority(&self) -> i8 {
        match *self {
            InfixBinaryOP::Add | InfixBinaryOP::Sub => 0,
            InfixBinaryOP::Mul | InfixBinaryOP::Div => 1,
            _ => -1,
        }
    }
}

impl From<Token> for UnaryOP {
    fn from(value: Token) -> Self {
        match value {
            Token::Sub => Self::Sub,
            op => panic!("invalid unary op: {:?}", op),
        }
    }
}

impl From<Token> for BinaryOP {
    fn from(value: Token) -> Self {
        match value {
            Token::Add => Self::Add,
            Token::Sub => Self::Sub,
            Token::Mul => Self::Mul,
            Token::Div => Self::Div,
            Token::Equal => Self::Equal,
            Token::NotEq => Self::NotEq,
            Token::Greater => Self::Greater,
            Token::Less => Self::Less,
            Token::GreEq => Self::GreEq,
            Token::LesEq => Self::LesEq,
            _ => panic!("invalid binary op!"),
        }
    }
}
