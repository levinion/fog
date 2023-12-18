use std::{fs::File, path::PathBuf};

use crate::{builder, complier, vm::VM, CONFIGURE};

pub async fn run(file: &Option<String>, debug: &bool) {
    let ir = if let Some(file) = file {
        // If file path is given, then complie the single file. Not support now.
        complier::complie_file(file, None).into()
    } else {
        // If frog file exists, parse it as ir. Or complie the project.
        let frog = get_frog_path();
        if frog.exists() {
            let file = File::open(frog.as_path()).unwrap();
            bincode::deserialize_from(file).unwrap()
        } else {
            complier::complie("src")
        }
    };
    // If debug mode is on, output the bytecodes in json format.
    if *debug {
        println!("{}", serde_json::to_string_pretty(&ir).unwrap());
    }
    // execute the ir.
    let mut vm = VM::new(ir);
    vm.execute().await;
}

pub fn new(name: &str) {
    builder::init_project(name);
}

pub fn build() {
    let ir = complier::complie("src");
    ir.build();
}

fn get_frog_path() -> PathBuf {
    let name = CONFIGURE.config.name.clone() + ".frog";
    let mut frog = PathBuf::from("bin");
    frog.push(&name);
    frog
}
