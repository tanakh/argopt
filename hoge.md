# Getting Started

```rust
fn connect(host: String, port: i64) {
    dbg!(host, port);
}
```

# Subcommand


# Easy Verbosity Level Handling

```
use argopt::cmd;
use log::*;

#[cmd(verbose)]
fn main() {
    error!("This is error");
    warn!("This is warn");
    info!("This is info");
    debug!("This is debug");
    trace!("This is trace");
}
```

```text
$ cargo run --help
test-verbose 0.1.0

USAGE:
    test-verbose [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv, etc.)
```

```text
$ cargo run
This is error

$ cargo run -- -v
This is error
This is warn

$ cargo run -- -vv
This is error
This is warn
This is info

$ cargo run -- -vvv
This is error
This is warn
This is info
This is debug

$ cargo run -- -vvvv
This is error
This is warn
This is info
This is debug
This is trace
```
