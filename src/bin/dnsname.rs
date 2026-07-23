use rgbdns::{RData, RecordType, client};
use std::net::Ipv4Addr;

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsname: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let address: Ipv4Addr = argument
            .parse()
            .map_err(|_| rgbdns::Error::Format("invalid IPv4 address"))?;
        let octets = address.octets();
        let name = format!(
            "{}.{}.{}.{}.in-addr.arpa",
            octets[3], octets[2], octets[1], octets[0]
        )
        .parse()?;
        let response = client::recursive(name, RecordType::Ptr)?;
        for target in response
            .answers
            .iter()
            .filter_map(|record| match &record.data {
                RData::Name(RecordType::Ptr, target) => Some(target),
                _ => None,
            })
        {
            print!("{target}");
        }
        println!();
    }
    Ok(())
}
