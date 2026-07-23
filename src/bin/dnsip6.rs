use rgbdns::{RData, RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsip6: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let response = client::recursive(argument.parse()?, RecordType::Aaaa)?;
        for address in response
            .answers
            .iter()
            .filter_map(|record| match record.data {
                RData::Aaaa(address) => Some(address),
                _ => None,
            })
        {
            print!("{address} ");
        }
        println!();
    }
    Ok(())
}
