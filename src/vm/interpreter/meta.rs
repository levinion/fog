use std::{collections::HashMap, process::exit};

use crate::core::value::{Args, Value};

pub fn init_global_table() -> HashMap<String, Value> {
    let mut global = HashMap::new();
    global.insert("println".to_string(), Value::Fn(lib_println));
    global.insert("print".to_string(), Value::Fn(lib_print));
    global.insert("exit".to_string(), Value::Fn(lib_exit));
    global
}

pub fn lib_println(args: Args) -> i32 {
    for v in args.iter() {
        println!("{}", v);
    }
    0
}

pub fn lib_print(args: Args) -> i32 {
    for v in args.iter() {
        print!("{}", v);
    }
    0
}

#[allow(unused)]
pub fn lib_exit(args: Args) -> i32 {
    exit(0)
}
