use rgbdns::{RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsqr: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 {
        return Err(rgbdns::Error::Format("usage: dnsqr type name"));
    }
    let record_type = arguments[0].parse::<RecordType>()?;
    let name = arguments[1].parse()?;
    println!("{} {}:", record_type.code(), name);
    println!("{:#?}", client::recursive(name, record_type)?);
    Ok(())
}
