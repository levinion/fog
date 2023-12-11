use std::collections::HashMap;

use super::block::Block;

pub struct IR {
    pub blocks: Vec<Block>,
}

impl From<Vec<Block>> for IR {
    fn from(value: Vec<Block>) -> Self {
        Self { blocks: value }
    }
}

impl From<IR> for HashMap<String, Block> {
    fn from(value: IR) -> Self {
        let mut map = HashMap::new();
        value.blocks.into_iter().for_each(|block| {
            map.insert(block.name.clone(), block);
        });
        map
    }
}
