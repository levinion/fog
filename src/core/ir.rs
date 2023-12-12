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
        walk_blocks(&mut map, value.blocks, "".into());
        map
    }
}

fn walk_blocks(map: &mut HashMap<String, Block>, blocks: Vec<Block>, father_name: String) {
    if blocks.is_empty() {
        return;
    }
    for block in blocks {
        let new_name = if father_name.is_empty() {
            block.name.clone()
        } else {
            father_name.clone() + "::" + &block.name
        };
        walk_blocks(map, block.sub_blocks.clone(), new_name.clone());
        // dbg!(new_name.clone());
        map.insert(new_name.clone(), block.clone());
    }
}
