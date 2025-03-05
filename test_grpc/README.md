# grpc server client rust
tree
```
rustam@rustam-zenbook:~/rust-education/test_grpc$ tree -L 2 . 
.
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── proto
│   └── helloworld.proto
├── README.md
├── src
│   ├── client.rs
│   ├── helloworld.rs
│   ├── main.rs
│   └── server.rs

```
run server
```
rustam@rustam-zenbook:~/rust-education/test_grpc$ cargo run --bin server
warning: unused imports: `env` and `path::PathBuf`
 --> build.rs:1:11
  |
1 | use std::{env, path::PathBuf};
  |           ^^^  ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `test_grpc` (build script) generated 1 warning
   Compiling test_grpc v0.1.0 (/home/rustam/rust-education/test_grpc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.73s
     Running `target/debug/server`
Server listening on [::1]:50051
client client data
client vec = 3.4 5.6
client client data
client vec = 3.4 5.6
client client data
client vec = 3.4 5.6

```
run client
```
rustam@rustam-zenbook:~/rust-education/test_grpc$ cargo run --bin client
warning: unused imports: `env` and `path::PathBuf`
 --> build.rs:1:11
  |
1 | use std::{env, path::PathBuf};
  |           ^^^  ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `test_grpc` (build script) generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/client`
RESPONSE=HelloReply { message: "hello client data", id: 23, payload: [1.2, 2.4] }
```
