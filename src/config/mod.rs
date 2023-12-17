use self::dependencies::Dependencies;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub mod dependencies;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct _Config {
    pub name: String,
    pub version: String,
    pub dependencies: HashMap<String, Dependencies>,
}

impl _Config {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            version: "0.1.0".into(),
            dependencies: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    path: PathBuf,
    pub config: _Config,
}

impl Config {
    /// create config if not exists.
    pub fn create(path: &str, name: &str) -> Self {
        if File::open(path).is_ok() {
            panic!("config file already exists!");
        }
        let config = Config {
            path: path.into(),
            config: _Config::new(name),
        };
        config.write();
        config
    }

    pub fn init() -> Self {
        let filename = "fog.toml";
        let mut file = File::open(filename).expect("config file not found");
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let config: _Config = toml::from_str(&content).unwrap();
        Self {
            path: filename.into(),
            config,
        }
    }

    /// if config file not exists, create it, or write over it.
    pub fn write(&self) {
        let mut file = File::create(&self.path).unwrap();
        file.write_all(toml::to_string(&self.config).unwrap().as_bytes())
            .unwrap();
    }
}
