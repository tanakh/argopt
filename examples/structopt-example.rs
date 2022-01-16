use argopt::cmd;
use std::path::PathBuf;

/// A basic example
#[cmd]
fn main(
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[opt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[opt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Set speed
    #[opt(short, long, default_value_t = 42.0)]
    speed: f64,

    /// Output file
    #[opt(short, long)]
    output: PathBuf,

    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`.
    /// Number of cars
    #[opt(short = 'c', long)]
    nb_cars: Option<i32>,

    /// admin_level to consider
    #[opt(short, long)]
    level: Vec<String>,

    /// Files to process
    #[opt(name = "FILE")]
    files: Vec<PathBuf>,
) {
    dbg!(debug, verbose, speed, output, nb_cars, level, files);
}
