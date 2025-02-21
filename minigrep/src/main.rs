use std::env;
use std::fs;


struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {

        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

fn parse_config(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }

    let query = args[1].clone();
    let file_path = args[2].clone();
    
    return Config { query, file_path };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    let content = fs::read_to_string(config.file_path).expect("file read error");
    println!("content first time from function = {content}");

    let config = Config::new(&args);
    println!("config from new operator: {}", config.file_path);
}
