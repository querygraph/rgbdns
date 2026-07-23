use rgbdns::multilog::{self, Config};
use std::io::BufReader;

fn main() {
    if let Err(error) = run() {
        eprintln!("multilog: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let config = Config::parse(&arguments)?;
    multilog::run(&config, BufReader::new(std::io::stdin()))?;
    Ok(())
}
