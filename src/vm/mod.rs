mod interpreter;
mod manager;
use anyhow::Result;

use self::manager::Manager;
use crate::core::{ir::IR, namespace::NameSpace};

pub struct VM {
    manager: Manager,
}

impl VM {
    pub fn new(ir: IR) -> Self {
        Self {
            manager: Manager::new(ir),
        }
    }

    pub async fn execute(&mut self) -> Result<()> {
        self.manager
            .exec("main::main", vec![], NameSpace::new("main"))
            .await
    }
}
