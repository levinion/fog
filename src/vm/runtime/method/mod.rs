use std::collections::HashMap;

use crate::core::value::{Args, Type};

pub fn init_method_table() -> HashMap<Type, fn(Args) -> i32> {
    let map = HashMap::new();
    // TODO:Impl this
    map
}

fn init_string_method() {}
