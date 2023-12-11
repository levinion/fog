use std::collections::HashMap;

use super::block::Block;

#[derive(Default)]
pub struct IR {
    pub blocks: Vec<Block>,
}

impl IR {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn import(mut self, block: Block) -> Self {
        self.blocks.push(block);
        self
    }
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
            block.sub_blocks.iter().for_each(|sub_block| {
                map.insert(sub_block.name.clone(), sub_block.clone());
            });
            map.insert(block.name.clone(), block);
        });
        map
    }
}
