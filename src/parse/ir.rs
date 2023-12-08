use crate::core::{bytecode::ByteCode, value::Value};

pub struct IR {
    pub byte_codes: Vec<ByteCode>,
    pub constants: Vec<Value>,
    pub locals: Vec<String>,
    pub pc: usize,
}

impl IR {
    pub fn go_ahead(&mut self) -> Option<&ByteCode> {
        let code = self.byte_codes.get(self.pc);
        self.pc += 1;
        code
    }

    pub fn jump_block(&mut self) {
        let mut count = 0;
        loop {
            let code = self.go_ahead();
            match code {
                Some(&ByteCode::EnterBlock) => count += 1,
                Some(&ByteCode::LeaveBlock) => {
                    count -= 1;
                    if count == 0 {
                        break;
                    }
                }
                Some(_) => {}
                None => panic!("unexpected eos!"),
            }
        }
    }

    pub fn debug(&self) {
        println!("{:#?}", self.byte_codes);
        println!("{:?}", self.constants);
        println!("{:?}", self.locals);
    }
}
