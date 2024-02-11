use std::sync::Arc;

use crate::core::{token::TokenVal, value::Type};

use super::Parser;

impl Parser {
    // next token should be that, or it will panic
    pub fn assert_next(&mut self, value: TokenVal) {
        let token = self.stream.next();
        if token.0.val != value {
            panic!(
                "expect {value:?}! Found {:?} in {:?}",
                token.0.val,
                format!("{}-{}", token.0.start, token.0.end)
            )
        }
    }

    // pub fn assert_next_string(&mut self) -> String {
    //     let token = self.stream.next();
    //     if let TokenVal::String(s) = token.0.val.clone() {
    //         s
    //     } else {
    //         panic!(
    //             "expect a string! Found {:?} in {:?}",
    //             token.0.val,
    //             format!("{}-{}", token.0.start, token.0.end)
    //         )
    //     }
    // }

    pub fn assert_next_type(&mut self) -> Type {
        let token = self.stream.next();
        if let TokenVal::Type(s) = token.0.val.clone() {
            s
        } else {
            panic!(
                "expect a name! Found {:?} in {:?}",
                token.0.val,
                format!("{}-{}", token.0.start, token.0.end)
            )
        }
    }

    pub fn assert_next_name(&mut self) -> Arc<String> {
        let token = self.stream.next();
        if let TokenVal::Name(s) = token.0.val.clone() {
            s
        } else {
            panic!(
                "expect a name! Found {:?} in {:?}",
                token.0.val,
                format!("{}-{}", token.0.start, token.0.end)
            )
        }
    }
}
