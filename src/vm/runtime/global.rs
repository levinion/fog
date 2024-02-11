use std::{collections::HashMap, process::exit, sync::Arc, time::Duration};

use anyhow::{anyhow, Result};

use crate::core::{
    block::Block,
    value::{Args, Value},
};

pub enum GlobalItem {
    Meta(Meta),
    Block(Arc<Block>),
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Meta(pub fn(Args, &Block) -> Result<Value>);

pub fn init_global_table() -> HashMap<String, GlobalItem> {
    let mut global: HashMap<String, GlobalItem> = HashMap::new();
    global.insert("@println".to_string(), GlobalItem::Meta(Meta(lib_println)));
    global.insert("@print".to_string(), GlobalItem::Meta(Meta(lib_print)));
    global.insert("@exit".to_string(), GlobalItem::Meta(Meta(lib_exit)));
    global.insert("@sleep".to_string(), GlobalItem::Meta(Meta(lib_sleep)));
    global.insert("@type".to_string(), GlobalItem::Meta(Meta(lib_type)));
    global.insert("@debug".to_string(), GlobalItem::Meta(Meta(lib_debug)));
    global.insert("@sh".to_string(), GlobalItem::Meta(Meta(lib_sh)));
    global
}

pub fn lib_println(args: Args, _block: &Block) -> Result<Value> {
    for v in args.iter() {
        println!("{}", v);
    }
    Ok(Value::Void(()))
}

pub fn lib_print(args: Args, _block: &Block) -> Result<Value> {
    for v in args.iter() {
        println!("{}", v);
    }
    Ok(Value::Void(()))
}

pub fn lib_exit(_args: Args, _block: &Block) -> Result<Value> {
    exit(0)
}

pub fn lib_sleep(args: Args, _block: &Block) -> Result<Value> {
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

pub fn lib_type(args: Args, _block: &Block) -> Result<Value> {
    if args.len() != 1 {
        return Err(anyhow!("Args Length Error: expect 1, found {}", args.len()));
    }
    let value = &args[0];
    Ok(Value::Type(value.typ()))
}

pub fn lib_debug(_args: Args, block: &Block) -> Result<Value> {
    println!("{:?}", block);
    Ok(Value::Void(()))
}

pub fn lib_sh(args: Args, _block: &Block) -> Result<Value> {
    if args.len() != 1 {
        return Err(anyhow!("Args Length Error: expect 1, found {}", args.len()));
    }
    if let Value::String(s) = &args[0] {
        let s = s.split(' ').collect::<Vec<_>>();
        std::process::Command::new(s[0])
            .args(&s[1..])
            .spawn()
            .unwrap();
        Ok(Value::Void(()))
    } else {
        Err(anyhow!(
            "Args Type Error: expect int, found {:?}",
            &args[0].typ()
        ))
    }
}
