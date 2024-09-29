use std::env::args;

pub mod parse_args;
pub mod remote;
pub mod udp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args: Vec<String> = args().collect();

    let mut parsed = crate::parse_args::Args::new();
    parsed.parse(args)?;
    Ok(())
}
