use std::io;
use rand::Rng;

fn main() {

    println!("Start game");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("rand = {secret_number}");

    println!("number = {}", guess);

}
