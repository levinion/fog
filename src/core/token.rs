use super::op::InfixBinaryOP;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    String(String), // "string"
    Name(String),   // constant/variable name
    ParL,           // (
    ParR,           // )
    Eos,            // end of input
    Let,            // let: define a variable
    Assign,         // =
    Equal,          // ==
    NotEq,          // !=
    Greater,        // >
    Less,           // <
    GreEq,          // >=
    LesEq,          // <=
    Comma,          // ,
    Bool(bool),     // true or false
    If,             // if
    Else,           // else
    CurlyL,         // {
    CurlyR,         // }
    Int(i64),       // 1
    Float(f64),     // 1.0
    Sub,            // -
    Add,            // +
    Mul,            // *
    Div,            // /
    Excl,           // !
    SemiColon,      // ;
    Fn,             // fn
    At,             // @
}

impl From<InfixBinaryOP> for Token {
    fn from(value: InfixBinaryOP) -> Self {
        match value {
            InfixBinaryOP::Add => Token::Add,
            InfixBinaryOP::Sub => Token::Sub,
            InfixBinaryOP::Mul => Token::Mul,
            InfixBinaryOP::Div => Token::Div,
            InfixBinaryOP::Equal => Token::Equal,
            InfixBinaryOP::NotEq => Token::NotEq,
            InfixBinaryOP::Greater => Token::Greater,
            InfixBinaryOP::Less => Token::Less,
            InfixBinaryOP::GreEq => Token::GreEq,
            InfixBinaryOP::LesEq => Token::LesEq,
            op => panic!("invalid op: {op:?}"),
        }
    }
}
