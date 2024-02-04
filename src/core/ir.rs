use std::collections::HashMap;

use crate::complier::optimizer;

use super::block::Block;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct IR1(pub Vec<Block>);

impl From<Vec<Block>> for IR1 {
    fn from(value: Vec<Block>) -> Self {
        Self(value)
    }
}

impl From<Block> for IR1 {
    fn from(value: Block) -> Self {
        Self(vec![value])
    }
}

impl From<IR1> for HashMap<String, Block> {
    fn from(value: IR1) -> Self {
        let mut map = HashMap::new();
        build_blocks(&mut map, &value.0);
        map
    }
}

impl From<IR1> for IR2 {
    fn from(value: IR1) -> Self {
        optimizer::optimize(value)
    }
}

fn build_blocks(map: &mut HashMap<String, Block>, blocks: &[Block]) {
    if blocks.is_empty() {
        return;
    }
    for block in blocks {
        map.insert(block.full_name.clone(), block.clone());
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct IR2(pub HashMap<String, Block>);
