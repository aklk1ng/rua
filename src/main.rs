use std::env;
use std::fs::File;
use std::io::BufReader;

pub mod byte_code;
pub mod lex;
pub mod parse;
pub mod value;
pub mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} script", args[0]);
        return;
    }

    let file = File::open(&args[1]).unwrap();
    let proto = parse::ParseProto::load(BufReader::new(file));
    vm::ExeState::new().execute(&proto);
}
