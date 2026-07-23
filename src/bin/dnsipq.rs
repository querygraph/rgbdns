use rgbdns::{Name, RData, RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsipq: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let name: Name = argument.parse()?;
        let response = client::recursive(name.clone(), RecordType::A)?;
        print!("{name} ");
        for address in response
            .answers
            .iter()
            .filter_map(|record| match record.data {
                RData::A(address) => Some(address),
                _ => None,
            })
        {
            print!("{address} ");
        }
        println!();
    }
    Ok(())
}
