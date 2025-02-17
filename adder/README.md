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
