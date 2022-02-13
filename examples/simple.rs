/// Simple greeting program
#[argopt::cmd]
fn main(
    /// greeting message
    #[opt(short, long, default_value = "Hello")]
    message: String,
    /// your name
    name: String,
) {
    println!("{message}, {name}!");
}
