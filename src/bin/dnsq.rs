use rgbdns::{RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsq: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 3 {
        return Err(rgbdns::Error::Format("usage: dnsq type name server"));
    }
    let server = client::server_address(&arguments[2])?;
    let response = client::query(
        arguments[1].parse()?,
        arguments[0].parse::<RecordType>()?,
        false,
        &[server],
    )?;
    println!("{response:#?}");
    Ok(())
}
