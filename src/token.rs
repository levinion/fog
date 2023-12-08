#[derive(PartialEq, Debug)]
pub enum Token {
    String(String), // "string"
    Name(String),   // constant/variable name
    ParL,           // (
    ParR,           // )
    Eos,            // end of input
    Let,            // let: define a variable
    Assign,         // =
    Equal,          // ==
    Comma,          // ,
    Bool(bool),     // true or false
    If,             // if
    CurlyL,         // {
    CurlyR,         // }
}
