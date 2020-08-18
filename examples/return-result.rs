use argopt::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[subcmd]
fn foo() -> Result<()> {
    Ok(())
}

#[subcmd]
fn bar() -> Result<()> {
    Err("error")?
}

#[cmd_group(commands = [foo, bar])]
fn main() -> Result<()> {}
