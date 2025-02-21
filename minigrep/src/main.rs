use std::env;
use std::fs;


struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    return Config { query, file_path };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    let content = fs::read_to_string(config.file_path).expect("file read error");
    println!("content = {content}");
}
