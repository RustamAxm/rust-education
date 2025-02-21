use std::env;

fn main() {
    let path = env::var("PATH");
    println!("{:?}", path);
    println!("env val ignore case = {}", env::var("IGNORE_CASE").is_ok());
}
