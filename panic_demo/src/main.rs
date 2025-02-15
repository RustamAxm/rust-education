use std::fs::File;
use std::io::{self, Read};
use panic_demo::Guess;

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hi.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


fn main() {

    let f_res = File::open("hi.txt");
    let f = match f_res {
        Ok(file) => file,
        Err(error) => panic!("file oprn problem {error:?}"),
    };
    let s = read_username_from_file().unwrap();
    println!("{s}");

    let g = Guess::new(5);
    dbg!(&g);
    let g2 = Guess::new(500);
    dbg!(&g2);
}
