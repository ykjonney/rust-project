use hello_macro::HelloMacro;
use hello_macro_derive::Builder;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Sunfei;
#[warn(dead_code)]
#[derive(HelloMacro)]
struct Sunface {
    name: String,
    age: u8,
}

#[derive(Builder, Debug)]
struct Command {
    executable: String,
    args: Option<Vec<String>>,
}

fn main() {
    // Sunfei::hello_macro();
    // Sunface::hello_macro();
    let command = Command::builder().
        executable("hello").
        // args(vec!["world".to_string()]).
        finish().unwrap();
    println!("{command:?}");
}