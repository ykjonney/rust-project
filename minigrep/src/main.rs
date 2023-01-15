use std::{env, process};
use minigrep::Config;

fn main() {
    let args = env::args();
    let config =Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e)=minigrep::run(config){
        println!("application error {}",e);
        process::exit(1);
    }

    // let mut stack = Vec::new();
    // stack.push(1);
    // stack.push(2);
    // stack.push(3);
    // while let Some(top) = stack.pop() {
    //     println!("{}",top);
    // }

}


