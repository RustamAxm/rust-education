# demo for args
```
rustam@rustam-zenbook:~/rust-education/minigrep$ cargo run -- -a -b in.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep -a -b in.txt`
-a -b in.txt
thread 'main' panicked at src/main.rs:12:44:
file read error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
rustam@rustam-zenbook:~/rust-education/minigrep$ echo "test file " >> in.txt
rustam@rustam-zenbook:~/rust-education/minigrep$ cargo run -- -a -b in.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep -a -b in.txt`
-a -b in.txt
content = test file 

[src/main.rs:14:5] args = [
    "target/debug/minigrep",
    "-a",
    "-b",
    "in.txt",
]
```
