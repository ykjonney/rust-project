use std::env;
use std::fs::File;
use std::io::BufReader;
mod lex;
mod value;
mod vm;
mod parse;
mod bytecode;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <lua file>", args[0]);
        return;
    }

    let lua_file = &args[1];
    let file = File::open(lua_file).unwrap();
    let input = BufReader::new(file);

    let proto = parse::ParseProto::load(input);
    let mut exe_state = vm::ExeState::new();
    exe_state.execute(&proto);
}