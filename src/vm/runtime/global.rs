use std::{collections::HashMap, process::exit, time::Duration};

use anyhow::{anyhow, Result};

use crate::core::{
    block::Block,
    value::{Args, Value},
};

pub enum GlobalItem {
    Meta(Meta),
    Block(Block),
}

pub type Meta = fn(Args) -> Result<Value>;

pub fn init_global_table() -> HashMap<String, GlobalItem> {
    let mut global: HashMap<String, GlobalItem> = HashMap::new();
    global.insert("@println".to_string(), GlobalItem::Meta(lib_println));
    global.insert("@print".to_string(), GlobalItem::Meta(lib_print));
    global.insert("@exit".to_string(), GlobalItem::Meta(lib_exit));
    global.insert("@sleep".to_string(), GlobalItem::Meta(lib_sleep));
    global.insert("@type".to_string(), GlobalItem::Meta(lib_type));
    global
}

pub fn lib_println(args: Args) -> Result<Value> {
    for v in args.iter() {
        println!("{}", v);
    }
    Ok(Value::Void(()))
}

pub fn lib_print(args: Args) -> Result<Value> {
    for v in args.iter() {
        println!("{}", v);
    }
    Ok(Value::Void(()))
}

pub fn lib_exit(_args: Args) -> Result<Value> {
    exit(0)
}

pub fn lib_sleep(args: Args) -> Result<Value> {
    if args.len() != 1 {
        return Err(anyhow!("Args Length Error: expect 1, found {}", args.len()));
    }
    let seconds = &args[0];
    if let Value::Int(i) = seconds {
        std::thread::sleep(Duration::from_secs(i.abs_diff(0)));
        Ok(Value::Void(()))
    } else {
        Err(anyhow!(
            "Args Type Error: expect int, found {:?}",
            seconds.typ()
        ))
    }
}

pub fn lib_type(args: Args) -> Result<Value> {
    if args.len() != 1 {
        return Err(anyhow!("Args Length Error: expect 1, found {}", args.len()));
    }
    let value = &args[0];
    Ok(Value::Type(value.typ()))
}
