
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {

        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

pub fn parse_config(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }

    let query = args[1].clone();
    let file_path = args[2].clone();

    return Config { query, file_path };
}


