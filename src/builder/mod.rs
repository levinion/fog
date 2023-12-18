use std::{io::Write, path::PathBuf};

use anyhow::Result;

use crate::config;

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
