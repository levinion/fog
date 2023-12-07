use crate::variable::Args;

pub fn lib_println(args: Args) -> i32 {
    for v in args.iter() {
        println!("{}", v);
    }
    0
}
