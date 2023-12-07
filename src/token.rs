#[derive(PartialEq, Debug)]
pub enum Token {
    String(String),
    Name(String),
    ParL,
    ParR,
    Eos,
}
