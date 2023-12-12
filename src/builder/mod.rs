use std::{io::Write, path::PathBuf};

pub struct Builder {}

impl Builder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init_project(&self, name: &str) {
        let root = PathBuf::from(name);
        let src = root.join("src");
        let main = src.join("main.fog");

        // create src
        std::fs::create_dir_all(src.as_path()).unwrap();
        let content = b"fn main(){\n\t@println(\"hello world!\");\n}";
        let mut main = std::fs::File::create(main).unwrap();
        main.write_all(content).unwrap();
    }
}
