use argopt::{cmd_group, subcmd};

/// Greeting command
#[subcmd]
fn greet(
    /// greeting message
    #[opt(short, long, default_value = "Hello")]
    greet: String,
    /// decolate message
    #[opt(long)]
    decolate: bool,
    /// Your name
    #[opt(name = "NAME")]
    name: String,
) {
    let msg = format!("{greet}, {name}!");
    let msg = if decolate {
        format!("*** {msg} ***")
    } else {
        msg
    };
    println!("{}", msg);
}

/// Connect command
#[subcmd]
fn connect(
    /// host name
    #[opt(long, default_value = "localhost")]
    host: String,
    /// port number
    #[opt(long, default_value_t = 8080)]
    port: i16,
) {
    println!("connect to {host}:{port}");
}

/// GetOpt example
#[subcmd(name = "getopt")]
fn getopt_example(
    /// output filename
    #[opt(short, long, default_value = "stdout")]
    output: String,
    /// input filename
    #[opt(short = 'c', default_value = "stdin")]
    input: String,
    /// library directory
    #[opt(short = 'L', long)]
    libdir: String,
) {
    println!("{output}, {input}, {libdir}");
}

/// Test program for subcommands
#[cmd_group(
    commands = [greet, connect, getopt_example],
    verbose,
)]
fn main() {}
