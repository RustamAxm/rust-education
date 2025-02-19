# for generate xml
```
cargo install --locked cargo-nextest
```
setup config 
```
rustam@rustam-zenbook:~/rust-education/adder$ cat .config/nextest.toml 
[profile.default.junit]  # this can be some other profile, too
path = "junit.xml"

# The name of the top-level "report" element in JUnit report. If aggregating
# reports across different test runs, it may be useful to provide separate names
# for each report.
report-name = "nextest-run"

# Whether standard output and standard error for passing tests should be stored in the JUnit report.
# Output is stored in the <system-out> and <system-err> elements of the <testcase> element.
store-success-output = true

# Whether standard output and standard error for failing tests should be stored in the JUnit report.
# Output is stored in the <system-out> and <system-err> elements of the <testcase> element.
#
# Note that if a description can be extracted from the output, it is always stored in the
# <description> element.
store-failure-output = true
```
run tests
```
cargo nextest run 
```

test results in target folder

```
rustam@rustam-zenbook:~/rust-education/adder$ tree target/nextest/default/
target/nextest/default/
└── junit.xml

```

## use dir organisation

```
rustam@rustam-zenbook:~/rust-education/adder$ tree -L 2 . 
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   └── lib.rs
├── target
│   ├── CACHEDIR.TAG
│   ├── debug
│   ├── nextest
│   └── tmp
└── tests
    └── integration_test.rs

7 directories, 6 files
rustam@rustam-zenbook:~/rust-education/adder$ cargo test 
warning: struct `Rectangle` is never constructed
  --> src/lib.rs:10:8
   |
10 | struct Rectangle {
   |        ^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: method `can_hold` is never used
  --> src/lib.rs:16:8
   |
15 | impl Rectangle {
   | -------------- method in this implementation
16 |     fn can_hold(&self, other: &Rectangle) -> bool {
   |        ^^^^^^^^

warning: field `value` is never read
  --> src/lib.rs:26:5
   |
25 | pub struct Guess {
   |            ----- field in this struct
26 |     value: i32,
   |     ^^^^^

warning: function `internal_adder` is never used
  --> src/lib.rs:29:4
   |
29 | fn internal_adder(left: usize, right: usize) -> usize {
   |    ^^^^^^^^^^^^^^

warning: `adder` (lib) generated 4 warnings
warning: field `value` is never read
  --> src/lib.rs:26:5
   |
25 | pub struct Guess {
   |            ----- field in this struct
26 |     value: i32,
   |     ^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `adder` (lib test) generated 1 warning
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/adder-1acff74a746ada6a)

running 8 tests
test tests::failed ... FAILED
test tests::greater_than100 - should panic ... ok
test tests::greeting_contain_name ... ok
test tests::internal ... ok
test tests::it_works ... ok
test tests::it_works_ok_or_err ... ok
test tests::larger_can_hold_smaller ... ok
test tests::smaller_cannot_hold_lager ... ok

failures:

---- tests::failed stdout ----
thread 'tests::failed' panicked at src/lib.rs:55:9:
failed test
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::failed

test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
rustam@rustam-zenbook:~/rust-education/adder$ cargo test --test integration_test
warning: struct `Rectangle` is never constructed
  --> src/lib.rs:10:8
   |
10 | struct Rectangle {
   |        ^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: method `can_hold` is never used
  --> src/lib.rs:16:8
   |
15 | impl Rectangle {
   | -------------- method in this implementation
16 |     fn can_hold(&self, other: &Rectangle) -> bool {
   |        ^^^^^^^^

warning: field `value` is never read
  --> src/lib.rs:26:5
   |
25 | pub struct Guess {
   |            ----- field in this struct
26 |     value: i32,
   |     ^^^^^

warning: function `internal_adder` is never used
  --> src/lib.rs:29:4
   |
29 | fn internal_adder(left: usize, right: usize) -> usize {
   |    ^^^^^^^^^^^^^^

warning: `adder` (lib) generated 4 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running tests/integration_test.rs (target/debug/deps/integration_test-0416628598656de2)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
