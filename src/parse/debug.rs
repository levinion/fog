use super::Parser;

impl Parser {
    pub fn debug(&self) {
        println!("{:#?}", self.byte_codes);
        println!("{:?}", self.constants);
        println!("{:?}", self.locals);
    }
}
