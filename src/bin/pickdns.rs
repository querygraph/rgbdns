use rgbdns::{pick::Database, special};
use std::sync::Arc;

fn main() {
    if let Err(error) = run() {
        eprintln!("pickdns: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::env::var("DATA").unwrap_or_else(|_| "data.cdb".into());
    let database = Arc::new(Database::from_file(data)?);
    let handler =
        Arc::new(move |wire: &[u8], limit: usize, client| database.respond(wire, limit, client));
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    special::serve(&format!("{ip}:{port}"), handler)?;
    Ok(())
}
