use rgbdns::dnssec;
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 2 {
        eprintln!("usage: rgbsec-check [data.signed [dnssec]]");
        return ExitCode::from(100);
    }
    let input = Path::new(arguments.first().map_or("data.signed", String::as_str));
    let policy = Path::new(arguments.get(1).map_or("dnssec", String::as_str));
    match dnssec::check_file(input, policy) {
        Ok(statuses) => {
            for status in statuses {
                println!("{status}");
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("rgbsec-check: {error}");
            ExitCode::from(111)
        }
    }
}
