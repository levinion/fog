mod interpreter;
use interpreter::Interpreter;

use crate::complier::ir::IR;

pub struct VM;

impl VM {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&mut self, ir: IR) {
        let mut itpt = Interpreter::new(&ir);
        for block in &ir.blocks {
            if block.name == "main" {
                itpt.execute(block.clone());
            }
        }
    }
}
