use std::{fs::File, io::Write, path::PathBuf};

use anyhow::Result;

use crate::{config, core::ir::IR2, CONFIGURE};

pub fn init_project(name: &str) -> Result<()> {
    let root = PathBuf::from(name);
    let src = root.join("src");
    let main = src.join("main.fog");

    // create src and main.fog
    std::fs::create_dir_all(src.as_path())?;
    let content = b"fn main(){\n\t@println(\"hello world!\");\n}";
    let mut main = std::fs::File::create(main)?;
    main.write_all(content)?;

    // create fog.config
    let config_name = root.join("fog.toml");
    config::Config::create(config_name.to_str().unwrap(), name)?;
    Ok(())
}

pub fn build_ir(ir: &IR2) -> Result<()> {
    let bin = PathBuf::from("bin");
    std::fs::create_dir_all(bin.as_path())?;
    let name = CONFIGURE.config.name.clone() + ".frog";
    let path = bin.join(name);
    let mut file = File::create(path)?;
    file.write_all(&bincode::serialize(ir)?)?;
    Ok(())
}
