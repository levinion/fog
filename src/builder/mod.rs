use std::{io::Write, path::PathBuf};

use crate::config;

pub fn init_project(name: &str) {
    let root = PathBuf::from(name);
    let src = root.join("src");
    let main = src.join("main.fog");

    // create src and main.fog
    std::fs::create_dir_all(src.as_path()).unwrap();
    let content = b"fn main(){\n\t@println(\"hello world!\");\n}";
    let mut main = std::fs::File::create(main).unwrap();
    main.write_all(content).unwrap();

    // create fog.config
    let config_name = root.join("fog.toml");
    config::Config::create(config_name.to_str().unwrap(), name);
}
