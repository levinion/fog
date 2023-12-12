mod interpreter;
mod manager;
use self::manager::Manager;
use crate::core::ir::IR;

pub struct VM {
    manager: Manager,
}

impl VM {
    pub fn new(ir: IR) -> Self {
        Self {
            manager: Manager::new(ir),
        }
    }

    pub async fn execute(&mut self) {
        self.manager.exec("src::main::main", vec![]).await;
    }
}
