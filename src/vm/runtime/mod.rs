pub mod global;
use self::global::GlobalItem;
use crate::core::{ir::IR2, namespace::NameSpace};
use std::collections::HashMap;

pub struct Runtime {
    global_table: HashMap<String, GlobalItem>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            global_table: global::init_global_table(), // global functions defined here
        }
    }

    pub fn set_ir(&mut self, ir: IR2) -> &mut Self {
        let block_table: HashMap<String, GlobalItem> =
            ir.0.into_iter()
                .map(|(name, block)| (name, GlobalItem::Block(block)))
                .collect();
        self.global_table.extend(block_table);
        self
    }

    /// find the true block by name and its father's namespace.
    // TODO: add other namespace: std, dependencies...
    pub fn get_global_by_name(&self, name: &str, namespace: NameSpace) -> Option<&GlobalItem> {
        let relative_name = namespace.append(name).to_string();

        #[allow(clippy::single_element_loop)]
        for name in [&relative_name] {
            let r = self.global_table.get(name);
            if r.is_some() {
                return r;
            }
        }
        self.global_table.get(name)
    }
}
