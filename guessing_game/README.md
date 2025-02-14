# guessing game
```
rustam@rustam-zenbook:~/rust-education$ cargo --version 
cargo 1.84.1 (66221abde 2024-11-19)
rustam@rustam-zenbook:~/rust-education$ cargo new guessing_game
    Creating binary (application) `guessing_game` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
rustam@rustam-zenbook:~/rust-education$ cd guessing_game/
rustam@rustam-zenbook:~/rust-education/guessing_game$ cat Cargo.toml 
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

[dependencies]
rustam@rustam-zenbook:~/rust-education/guessing_game$ cat src/main.rs 
fn main() {
    println!("Hello, world!");
}
rustam@rustam-zenbook:~/rust-education/guessing_game$ ls 
Cargo.toml  src
```
