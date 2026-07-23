use rgbdns::{RData, RecordType, client};
use std::io::Write;

fn main() {
    if let Err(error) = run() {
        eprintln!("dnstxt: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    let mut stdout = std::io::stdout().lock();
    for argument in std::env::args().skip(1) {
        let response = client::recursive(argument.parse()?, RecordType::Txt)?;
        for chunk in response
            .answers
            .iter()
            .flat_map(|record| match &record.data {
                RData::Txt(chunks) => chunks.as_slice(),
                _ => &[],
            })
        {
            stdout.write_all(chunk)?;
        }
        stdout.write_all(b"\n")?;
    }
    Ok(())
}
