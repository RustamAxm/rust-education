use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Start game");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("rand = {secret_number}");
    
    loop {

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error in read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("win");
                break;
            }

        }
    }

}
