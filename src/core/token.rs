use std::{fmt::Display, rc::Rc, sync::Arc};

use super::{op::InfixBinaryOP, value::Type};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Offset {
    pub row: usize,
    pub col: usize,
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct _Token {
    pub val: TokenVal,
    pub start: Offset,
    pub end: Offset,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token(pub Rc<_Token>);

impl Token {
    pub fn new(val: TokenVal, start: Offset, end: Offset) -> Self {
        Self(Rc::new(_Token::new(val, start, end)))
    }
}

impl _Token {
    pub fn new(val: TokenVal, start: Offset, end: Offset) -> Self {
        Self { val, start, end }
    }

    pub fn infix_binary_op(&self) -> InfixBinaryOP {
        match &self.val {
            TokenVal::Add => InfixBinaryOP::Add,
            TokenVal::Sub => InfixBinaryOP::Sub,
            TokenVal::Mul => InfixBinaryOP::Mul,
            TokenVal::Div => InfixBinaryOP::Div,
            TokenVal::ParL => InfixBinaryOP::ParL,
            TokenVal::ParR => InfixBinaryOP::ParR,
            TokenVal::Equal => InfixBinaryOP::Equal,
            TokenVal::NotEq => InfixBinaryOP::NotEq,
            TokenVal::Greater => InfixBinaryOP::Greater,
            TokenVal::Less => InfixBinaryOP::Less,
            TokenVal::GreEq => InfixBinaryOP::GreEq,
            TokenVal::LesEq => InfixBinaryOP::LesEq,
            op => panic!("invalid infix binary op: {:?}", op),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenVal {
    String(Arc<String>), // "string"
    Name(Arc<String>),   // constant/variable name
    ParL,                // (
    ParR,                // )
    Eos,                 // end of input
    Let,                 // let: define a variable
    Assign,              // =
    Equal,               // ==
    NotEq,               // !=
    Greater,             // >
    Less,                // <
    GreEq,               // >=
    LesEq,               // <=
    Comma,               // ,
    Bool(bool),          // true or false
    If,                  // if
    Else,                // else
    CurlyL,              // {
    CurlyR,              // }
    Int(i64),            // 1
    Float(f64),          // 1.0
    Sub,                 // -
    Add,                 // +
    Mul,                 // *
    Div,                 // /
    Excl,                // !
    SemiColon,           // ;
    Fn,                  // fn
    Fog,                 // fog
    Dot,                 // .
    Colon,               // :
    Import,              // import
    Use,                 // use
    For,                 // loop
    RightArror,          // =>
    Return,              // return
    Type(Type),
}

impl From<InfixBinaryOP> for TokenVal {
    fn from(value: InfixBinaryOP) -> Self {
        match value {
            InfixBinaryOP::Add => TokenVal::Add,
            InfixBinaryOP::Sub => TokenVal::Sub,
            InfixBinaryOP::Mul => TokenVal::Mul,
            InfixBinaryOP::Div => TokenVal::Div,
            InfixBinaryOP::Equal => TokenVal::Equal,
            InfixBinaryOP::NotEq => TokenVal::NotEq,
            InfixBinaryOP::Greater => TokenVal::Greater,
            InfixBinaryOP::Less => TokenVal::Less,
            InfixBinaryOP::GreEq => TokenVal::GreEq,
            InfixBinaryOP::LesEq => TokenVal::LesEq,
            op => panic!("invalid op: {op:?}"),
        }
    }
}

pub trait IsMeta {
    fn is_meta(&self) -> bool;
}

impl IsMeta for String {
    fn is_meta(&self) -> bool {
        self.starts_with('@')
    }
}
