use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

use anyhow::Result;

use crate::CONFIGURE;

use super::block::Block;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct IR(Vec<Block>);

impl IR {
    pub fn build(&self) -> Result<()> {
        let bin = PathBuf::from("bin");
        std::fs::create_dir_all(bin.as_path())?;
        let name = CONFIGURE.config.name.clone() + ".frog";
        let path = bin.join(name);
        let mut file = File::create(path)?;
        file.write_all(&bincode::serialize(&self)?)?;
        Ok(())
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
