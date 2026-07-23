use rgbdns::{special, wall};
use std::sync::Arc;

fn main() {
    if let Err(error) = run() {
        eprintln!("walldns: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    special::serve(&format!("{ip}:{port}"), Arc::new(wall::respond))?;
    Ok(())
}
