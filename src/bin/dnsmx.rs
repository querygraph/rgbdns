use rgbdns::{RData, RecordType, client};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsmx: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let name = argument.parse()?;
        let response = client::recursive(name, RecordType::Mx)?;
        let mut found = false;
        for (preference, target) in
            response
                .answers
                .iter()
                .filter_map(|record| match &record.data {
                    RData::Mx(preference, target) => Some((preference, target)),
                    _ => None,
                })
        {
            println!("{preference} {target}");
            found = true;
        }
        if !found {
            println!("0 {argument}.");
        }
    }
    Ok(())
}
