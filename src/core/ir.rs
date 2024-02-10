use std::{collections::HashMap, sync::Arc};

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

impl From<IR1> for HashMap<String, Arc<Block>> {
    fn from(value: IR1) -> Self {
        let mut map = HashMap::new();
        build_blocks(&mut map, value.0);
        map
    }
}

impl From<IR1> for IR2 {
    fn from(value: IR1) -> Self {
        optimizer::optimize(value)
    }
}

fn build_blocks(map: &mut HashMap<String, Arc<Block>>, blocks: Vec<Block>) {
    if blocks.is_empty() {
        return;
    }
    for block in blocks {
        map.insert(block.full_name.clone(), Arc::new(block));
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct IR2(pub HashMap<String, Arc<Block>>);
