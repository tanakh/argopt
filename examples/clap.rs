use structopt::clap::{App, Arg};

fn main() {
    let matches = App::new("greet")
        .version("0.1.0")
        .about("Simple greeting program")
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .default_value("Hello"),
        )
        .arg(Arg::with_name("name").required(true))
        .get_matches();

    let message = matches.value_of("message").unwrap();
    let name = matches.value_of("name").unwrap();

    println!("{}, {}!", message, name);
}
