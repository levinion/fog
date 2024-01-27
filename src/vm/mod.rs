mod interpreter;
pub mod runtime;

use anyhow::{anyhow, Context, Result};

use self::{
    interpreter::Interpreter,
    runtime::{global::GlobalItem, Runtime},
};
use crate::{
    core::{ir::IR2, namespace::NameSpace},
    VM,
};

pub struct Vm {
    runtime: Runtime,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            runtime: Runtime::new(),
        }
    }

    pub async fn execute(&self) -> Result<()> {
        let item = self
            .runtime
            .get_global_by_name("main::main", NameSpace::new("main"))
            .context("name not found!")?;
        if let GlobalItem::Block(block) = item {
            let mut main_thread = Interpreter::new();
            main_thread.execute(block.clone(), vec![]).await?;
            Ok(())
        } else {
            Err(anyhow!("main::main is not a block!"))
        }
    }
}

pub fn init_global_vm(ir: IR2) {
    VM.get_or_init(|| {
        let mut vm = Vm::new();
        vm.runtime.set_ir(ir);
        vm
    });
}
