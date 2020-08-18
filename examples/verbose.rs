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
