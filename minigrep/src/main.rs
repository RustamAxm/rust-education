use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let a = &args[1];
    let b = &args[2];
    let file = &args[3];
    println!("{a} {b} {file}");

    let content = fs::read_to_string(file).expect("file read error");
    println!("content = {content}");
    dbg!(args);
}
