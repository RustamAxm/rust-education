use std::env;
use std::fs;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    let content = fs::read_to_string(config.file_path).expect("file read error");
    println!("content first time from function = {content}");

    let config = Config::new(&args);
    println!("config from new operator: {}", config.file_path);
}
