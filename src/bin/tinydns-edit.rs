use rgbdns::{Name, tinydns_edit};
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    path::Path,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("tinydns-edit: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 6 || arguments[2] != "add" {
        return Err("usage: tinydns-edit data data.new add \
             [ns|childns|host|alias|mx|host6|alias6] domain address"
            .into());
    }
    tinydns_edit::add(
        Path::new(&arguments[0]),
        Path::new(&arguments[1]),
        tinydns_edit::Mode::parse(&arguments[3])?,
        arguments[4].parse::<Name>()?,
        if matches!(arguments[3].as_str(), "host6" | "alias6") {
            tinydns_edit::Address::V6(arguments[5].parse::<Ipv6Addr>()?)
        } else {
            tinydns_edit::Address::V4(arguments[5].parse::<Ipv4Addr>()?)
        },
    )?;
    Ok(())
}
