# tcp hello

run server 
```
rustam@rustam-zenbook:~/rust-education/tcp-hello$ cargo run 
   Compiling tcp-hello v0.1.0 (/home/rustam/rust-education/tcp-hello)
warning: unused variable: `stream`
 --> src/main.rs:7:13
  |
7 |         let stream = stream.unwrap();
  |             ^^^^^^ help: if this is intentional, prefix it with an underscore: `_stream`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `tcp-hello` (bin "tcp-hello") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/tcp-hello`
Connection establish
Connection establish
Connection establish
```
for send use netcat 
```
rustam@rustam-zenbook:~/rust-education/tcp-hello$ cat hello.txt 
from file hi!
rustam@rustam-zenbook:~/rust-education/tcp-hello$ netcat 127.0.0.1 7878 < hello.txt 

```
