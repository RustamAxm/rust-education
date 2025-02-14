# guessing game
## init
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

## run game 

```
rustam@rustam-zenbook:~/rust-education/guessing_game$ cargo run 
   Compiling guessing_game v0.1.0 (/home/rustam/rust-education/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Start game
34
number = 34

```
## add crates
```
rustam@rustam-zenbook:~/rust-education/guessing_game$ cargo add rand 
    Updating crates.io index
      Adding rand v0.9.0 to dependencies
             Features:
             + alloc
             + os_rng
             + small_rng
             + std
             + std_rng
             + thread_rng
             - log
             - nightly
             - serde
             - simd_support
             - unbiased
    Updating crates.io index
     Locking 28 packages to latest compatible versions
      Adding bitflags v2.8.0
      Adding byteorder v1.5.0
      Adding cfg-if v1.0.0
      Adding getrandom v0.3.1
      Adding libc v0.2.169
      Adding ppv-lite86 v0.2.20
      Adding proc-macro2 v1.0.93
      Adding quote v1.0.38
      Adding rand v0.9.0
      Adding rand_chacha v0.9.0
      Adding rand_core v0.9.0
      Adding syn v2.0.98
      Adding unicode-ident v1.0.16
      Adding wasi v0.13.3+wasi-0.2.2
      Adding windows-targets v0.52.6
      Adding windows_aarch64_gnullvm v0.52.6
      Adding windows_aarch64_msvc v0.52.6
      Adding windows_i686_gnu v0.52.6
      Adding windows_i686_gnullvm v0.52.6
      Adding windows_i686_msvc v0.52.6
      Adding windows_x86_64_gnu v0.52.6
      Adding windows_x86_64_gnullvm v0.52.6
      Adding windows_x86_64_msvc v0.52.6
      Adding wit-bindgen-rt v0.33.0
      Adding zerocopy v0.7.35
      Adding zerocopy v0.8.17
      Adding zerocopy-derive v0.7.35
      Adding zerocopy-derive v0.8.17
rustam@rustam-zenbook:~/rust-education/guessing_game$ cat Cargo.toml 
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.9.0"
```
