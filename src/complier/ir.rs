use super::block::Block;

pub struct IR {
    pub blocks: Vec<Block>,
}

impl From<Vec<Block>> for IR {
    fn from(value: Vec<Block>) -> Self {
        Self { blocks: value }
    }
}
