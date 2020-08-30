/*!
This crate provides attribute macros for command-line argument parsing.

# Usage

Just by adding an attribute `#[cmd]` to a function, the function is converted to a command line program.

```
# use argopt::cmd;
#[cmd]
fn main(host: String, port: u16) {
    // ...
}
```

The output is:

```text
$ cargo run
error: The following required arguments were not provided:
    <host>
    <port>

USAGE:
    argopt-test <host> <port>
```

You can customize the behavior of arguments by annotating them with attributes.

```
# use argopt::cmd;
#[cmd]
fn main(
    #[opt(short = "h", long = "host")]
    host: String,
    #[opt(short, long, default_value = "80")]
    port: u16,
) {
    // ...
}
```

And you can add help messages by adding doccomments.

```
# use argopt::cmd;
/// Sample program
#[cmd]
fn main(
    /// Host name
    #[opt(short = "h", long = "host")]
    host: String,
    /// Port number
    #[opt(short, long, default_value = "80")]
    port: u16,
) {
    // ...
}
```

The output is:

```text
argopt-test 0.1.0
Sample program

USAGE:
    simple [OPTIONS] --host <host>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --host <host>    Host name
    -p, --port <port>    Port number [default: 80]
```

You can use the same options as [structopt](https://crates.io/crates/structopt).

# Subcommands

You can create sub commands by adding the attribute `#[subcmd]` to functions.

```
# use argopt::*;
# use std::path::PathBuf;
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
fn main() {}
```

# Easy Verbosity Level Handling

There is a feature that allows you to interact with the [log](https://crates.io/crates/log) crate and handle the verbosity level automatically.

```
# use argopt::cmd;
# use log::*;
#[cmd(verbose)]
fn main() {
    error!("This is error");
    warn!("This is warn");
    info!("This is info");
    debug!("This is debug");
    trace!("This is trace");
}
```

The output is:

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
*/

pub use argopt_impl::{cmd, cmd_group, subcmd};

#[doc(hidden)]
pub use structopt::StructOpt;
