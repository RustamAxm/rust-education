use std::io;

fn main() {

    println!("Start game");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read");
    println!("number = {}", guess);
}
