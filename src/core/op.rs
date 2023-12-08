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
}
