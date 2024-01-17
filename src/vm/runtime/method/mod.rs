use std::collections::HashMap;

use crate::core::{typ::Type, value::Args};

pub fn init_method_table() -> HashMap<Type, fn(Args) -> i32> {
    let map = HashMap::new();
    todo!();
    map
}

fn init_string_method() {}
