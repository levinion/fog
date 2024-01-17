use std::collections::HashMap;

use crate::complier::optimizer;

use super::block::Block;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct IR(Vec<Block>);

impl IR {
    pub fn optimize(self) -> HashMap<String, Block> {
        optimizer::optimize(self)
    }
}

impl From<Vec<Block>> for IR {
    fn from(value: Vec<Block>) -> Self {
        Self(value)
    }
}

impl From<Block> for IR {
    fn from(value: Block) -> Self {
        Self(vec![value])
    }
}

impl From<IR> for HashMap<String, Block> {
    fn from(value: IR) -> Self {
        let mut map = HashMap::new();
        build_blocks(&mut map, &value.0);
        map
    }
}

fn build_blocks(map: &mut HashMap<String, Block>, blocks: &[Block]) {
    if blocks.is_empty() {
        return;
    }
    for block in blocks {
        map.insert(block.full_name.clone(), block.clone());
        build_blocks(map, &block.sub_blocks);
    }
}
