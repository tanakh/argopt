use std::time::Duration;

/// Simple greeting program
#[argopt::cmd]
#[tokio::main]
async fn main(
    /// your name
    secs: u64,
) {
    println!("Sleep {secs} seconds");
    tokio::time::sleep(Duration::from_secs(secs)).await;
}
