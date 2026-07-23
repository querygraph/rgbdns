use rgbdns::{Name, rbl::Database, special};
use std::sync::Arc;

fn main() {
    if let Err(error) = run() {
        eprintln!("rbldns: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::env::var("DATA").unwrap_or_else(|_| "data.cdb".into());
    let base: Name = std::env::var("BASE")
        .map_err(|_| "BASE is required")?
        .parse()?;
    let database = Arc::new(Database::from_file(data)?);
    let handler =
        Arc::new(move |wire: &[u8], limit: usize, _| database.respond(&base, wire, limit));
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    let address = rgbdns::socket_address(&ip, &port)?;
    special::serve(&address.to_string(), handler)?;
    Ok(())
}
