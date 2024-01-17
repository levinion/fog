mod interpreter;
pub mod runtime;

use anyhow::Result;

use self::runtime::Runtime;
use crate::core::{ir::IR, namespace::NameSpace};

pub struct Vm {
    runtime: Runtime,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            runtime: Runtime::new(),
        }
    }

    pub async fn execute(&mut self, ir: IR) -> Result<()> {
        self.runtime
            .set_ir(ir)
            .exec("main::main", vec![], NameSpace::new("main"))
            .await
    }
}
