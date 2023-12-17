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

impl From<Block> for IR {
    fn from(value: Block) -> Self {
        Self {
            blocks: vec![value],
        }
    }
}

impl From<IR> for HashMap<String, Block> {
    fn from(value: IR) -> Self {
        let mut map = HashMap::new();
        build_blocks(&mut map, &value.blocks);
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
