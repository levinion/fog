pub enum ByteCode {
    LoadConst { index: usize },
    GetGlobal,
    CallFunction { argc: usize },
}
