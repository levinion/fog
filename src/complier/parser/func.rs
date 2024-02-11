use std::sync::Arc;

use crate::core::{
    bytecode::ByteCode,
    token::TokenVal,
    value::{Type, Value},
};

use super::Parser;

impl Parser {
    /// call normal function with name
    /// eg: print(a, b);
    pub fn call_function(&mut self) -> Vec<ByteCode> {
        let mut codes = vec![];
        let name = self.assert_next_name();
        self.assert_next(TokenVal::ParL);
        // get args
        let (mut args, argc) = self.load_exps();
        codes.append(&mut args);
        self.assert_next(TokenVal::ParR);
        // call function
        codes.push(self.load_name(name));
        codes.push(ByteCode::LoadName);
        codes.push(ByteCode::CallFunction(argc));
        codes
    }

    pub fn fog_call_function(&mut self) -> Vec<ByteCode> {
        let mut codes = vec![];
        self.stream.next();
        let name = self.assert_next_name();
        self.assert_next(TokenVal::ParL);
        let (mut args, argc) = self.load_exps();
        codes.append(&mut args);
        self.assert_next(TokenVal::ParR);
        codes.push(self.load_name(name));
        codes.push(ByteCode::LoadName);
        codes.push(ByteCode::FogCallFunction(argc));
        codes
    }

    // eg: value.method(exps);
    pub fn call_method(&mut self, should_load: bool, should_load_name: bool) -> Vec<ByteCode> {
        let mut codes = vec![];
        // load the value as the first arg
        if should_load {
            let value: Value = self.stream.next().into();
            codes.push(ByteCode::LoadValue(value)); // feel free to clone this!
            if should_load_name {
                codes.push(ByteCode::LoadName);
            }
        }
        self.assert_next(TokenVal::Dot);
        // keep the method name for later
        let name = self.assert_next_name();
        self.assert_next(TokenVal::ParL);
        // get other args
        let (mut args, argc) = self.load_exps();
        codes.append(&mut args);
        self.assert_next(TokenVal::ParR);
        codes.push(self.load_name(name));
        codes.push(ByteCode::LoadName);
        codes.push(ByteCode::CallFunction(argc + 1));
        codes
    }

    /// This function is used by parse_blocks.
    /// It should not be used in other position.
    /// eg: fn test(a:type,b:type){...} -> get \[("a",type),("b",type)\]
    pub fn parse_fn_args_to_vec(&mut self) -> Vec<(Arc<String>, Type)> {
        let mut args = vec![];
        self.assert_next(TokenVal::ParL);
        loop {
            let token = self.stream.look_ahead(1);
            match token.0.val.clone() {
                TokenVal::Name(name) => {
                    self.stream.next();
                    self.assert_next(TokenVal::Colon);
                    let typ = self.assert_next_type();
                    args.push((name, typ));
                }
                TokenVal::ParR => break,
                TokenVal::Comma => self.assert_next(TokenVal::Comma),
                _ => panic!("invalid token!"),
            }
        }
        self.assert_next(TokenVal::ParR);
        args
    }
}
