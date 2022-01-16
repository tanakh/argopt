
[![Crates.io](https://img.shields.io/crates/v/argopt.svg)](https://crates.io/crates/argopt)
[![Workflow Status](https://github.com/tanakh/argopt/workflows/Rust/badge.svg)](https://github.com/tanakh/argopt/actions?query=workflow%3A%22Rust%22)

This crate provides attribute macros for command-line argument parsing.

# Usage

Just by adding an attribute `#[cmd]` to a function, the function is converted to a command line program.

```rust,should_panic
#[argopt::cmd]
fn main(host: String, port: u16) {
    // ...
}
```

```text
$ cargo run
error: The following required arguments were not provided:
    <HOST>
    <PORT>

USAGE:
    argopt-test <HOST> <PORT>

For more information try --help
```

```text
$ cargo run -- --help
argopt-test 

USAGE:
    argopt-test <HOST> <PORT>

ARGS:
    <HOST>    
    <PORT>    

OPTIONS:
    -h, --help    Print help information
```

You can customize the behavior of arguments by annotating them with `#[opt(...)]` attributes.

```rust,should_panic
#[argopt::cmd]
fn main(
    #[opt(short = 'h', long = "host")]
    host: String,
    #[opt(short, long, default_value_t = 80)]
    port: u16,
) {
    // ...
}
```

And you can add help messages by adding doccomments.

```rust,should_panic
/// Sample program
#[argopt::cmd]
fn main(
    /// Host name
    #[opt(short = 'h', long = "host")]
    host: String,
    /// Port number
    #[opt(short, long, default_value_t = 80)]
    port: u16,
) {
    // ...
}
```

You can also use the `#[opt(...)]` attribute to customize the behavior of an application.

```rust,should_panic
/// Sample program
#[argopt::cmd]
#[opt(author, version, about, long_about = None)]
fn main(
    /// Host name
    #[opt(short = 'h', long = "host")]
    host: String,
    /// Port number
    #[opt(short, long, default_value_t = 80)]
    port: u16,
) {
    // ...
}
```

```text
$ cargo run -- --help
argopt-test 0.1.0
Sample program

USAGE:
    argopt-test [OPTIONS] --host <HOST>

OPTIONS:
    -h, --host <HOST>    Host name
        --help           Print help information
    -p, --port <PORT>    Port number [default: 80]
    -V, --version        Print version information
```

The available options are the same as those of [clap::Parser](https://crates.io/crates/clap).

# Subcommands

You can create sub commands by adding the attribute `#[subcmd]` to functions.

```rust,should_panic
use argopt::{subcmd, cmd_group};
use std::path::PathBuf;

#[subcmd]
fn add(
    #[opt(short)]
    interactive: bool,
    #[opt(short)]
    patch: bool,
    files: Vec<PathBuf>,
) {
    // ...
}

#[subcmd]
fn commit(
    #[opt(short)]
    message: Option<String>,
    #[opt(short)]
    all: bool,
) {
    // ...
}

#[cmd_group(commands = [add, commit])]
#[opt(author, version, about, long_about = None)]
fn main() {}
```

# Easy Verbosity Level Handling

There is a feature that allows you to interact with the [log](https://crates.io/crates/log) crate and handle the verbosity level automatically.

```rust
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
