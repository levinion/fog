#![allow(dead_code)]
use super::token::TokenVal;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum UnaryOP {
    Sub = 0, // -
    Excl,    // !
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
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
    pub fn priority(&self) -> u8 {
        match *self {
            InfixBinaryOP::Add | InfixBinaryOP::Sub => 2,
            InfixBinaryOP::Mul | InfixBinaryOP::Div => 4,
            InfixBinaryOP::Equal
            | InfixBinaryOP::NotEq
            | InfixBinaryOP::Greater
            | InfixBinaryOP::Less
            | InfixBinaryOP::GreEq
            | InfixBinaryOP::LesEq => 8,
            InfixBinaryOP::ParR | InfixBinaryOP::ParL => 1,
        }
    }
}

impl From<TokenVal> for UnaryOP {
    fn from(value: TokenVal) -> Self {
        match value {
            TokenVal::Sub => Self::Sub,
            TokenVal::Excl => Self::Excl,
            op => panic!("invalid unary op: {:?}", op),
        }
    }
}

impl From<TokenVal> for BinaryOP {
    fn from(value: TokenVal) -> Self {
        match value {
            TokenVal::Add => Self::Add,
            TokenVal::Sub => Self::Sub,
            TokenVal::Mul => Self::Mul,
            TokenVal::Div => Self::Div,
            TokenVal::Equal => Self::Equal,
            TokenVal::NotEq => Self::NotEq,
            TokenVal::Greater => Self::Greater,
            TokenVal::Less => Self::Less,
            TokenVal::GreEq => Self::GreEq,
            TokenVal::LesEq => Self::LesEq,
            _ => panic!("invalid binary op!"),
        }
    }
}
