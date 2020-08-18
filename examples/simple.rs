use argopt::cmd;

/// Simple greeting program
#[argopt::cmd]
fn main(
    /// greeting message
    #[opt(short, long, default_value = "Hello")]
    message: String,
    /// your name
    //
    name: String,
) {
    println!("{}, {}!", message, name);
}

// #[cmd]
// fn main(host: String, port: i16) {
//     dbg!(host, port);
// }

// #[cmd]
// fn main(
//     #[opt(default_value = "localhost")] host: String,
//     #[opt(default_value = "8080")] port: i64,
// ) {
//     dbg!(host, port);
// }

// #[cmd]
// fn main(
//     /// host name
//     #[opt(default_value = "localhost")]
//     host: String,
//     /// port number
//     #[opt(default_value = "8080")]
//     port: i64,
// ) {
//     dbg!(host, port);
// }
