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
    NotEqual,       // !=
    Comma,          // ,
    Bool(bool),     // true or false
    If,             // if
    CurlyL,         // {
    CurlyR,         // }
    Int(i64),       // 1
    Float(f64),     // 1.0
    Sub,            // -
    Add,            // +
    Mul,            // *
    Div,            // /
    Excl,           // !
}
