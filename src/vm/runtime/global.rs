use std::{collections::HashMap, process::exit, time::Duration};

use crate::{
    complier::complie_string,
    core::{
        block::Block,
        value::{Args, Value},
    },
};

pub enum GlobalItem {
    Meta(Meta),
    Block(Block),
}

pub type Meta = fn(Args) -> i32;

pub fn init_global_table() -> HashMap<String, GlobalItem> {
    let mut global: HashMap<String, GlobalItem> = HashMap::new();
    global.insert("@println".to_string(), GlobalItem::Meta(lib_println));
    global.insert("@print".to_string(), GlobalItem::Meta(lib_print));
    global.insert("@exit".to_string(), GlobalItem::Meta(lib_exit));
    global.insert("@sleep".to_string(), GlobalItem::Meta(lib_sleep));
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

pub fn lib_exit(_args: Args) -> i32 {
    exit(0)
}

pub fn lib_sleep(args: Args) -> i32 {
    if args.len() != 1 {
        return 1;
    }
    let seconds = &args[0];
    if let Value::Int(i) = seconds {
        std::thread::sleep(Duration::from_secs(i.abs_diff(0)));
        0
    } else {
        1
    }
}

pub fn lib_type(args: Args) -> i32 {
    if args.len() != 1 {
        return 1;
    }
    let value = &args[0];
    value.typ();
    todo!();
}

pub fn eval(args: Args) -> i32 {
    if args.len() != 1 {
        return 1;
    }
    let code = &args[0];
    if let Value::String(s) = code {
        // TODO: rebuild the package system.
        todo!();
        // complie_string(&s, father)
    }
    todo!()
}
