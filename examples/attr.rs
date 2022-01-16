#[argopt::cmd]
#[opt(
    name = "attribute-test",
    author,
    version,
    about,
    long_about = None,
    color = argopt::clap::ColorChoice::Never,
)]
fn main(
    /// Name of the person to greet
    #[opt(short, long)]
    name: String,

    /// Number of times to greet
    #[opt(short, long, default_value_t = 1)]
    count: u8,
) {
    for _ in 0..count {
        println!("Hello {name}!")
    }
}
