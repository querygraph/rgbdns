use rgbdns::{Name, axfr};
use std::{net::SocketAddr, path::Path};

fn main() {
    if let Err(error) = run() {
        eprintln!("axfr-get: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args().skip(1);
    let zone: Name = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?
        .parse()?;
    let server_text = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?;
    let server: SocketAddr = if server_text.contains(':') {
        server_text.parse()?
    } else {
        format!("{server_text}:53").parse()?
    };
    let output = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?;
    let temporary = arguments
        .next()
        .ok_or("usage: axfr-get zone server[:port] output temporary")?;
    if arguments.next().is_some() {
        return Err("usage: axfr-get zone server[:port] output temporary".into());
    }
    let records = axfr::fetch(server, zone)?;
    axfr::write_tinydns(&records, Path::new(&output), Path::new(&temporary))?;
    Ok(())
}
