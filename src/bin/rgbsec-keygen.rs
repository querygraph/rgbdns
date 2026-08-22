use rgbdns::{Name, dnssec};
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 {
        eprintln!("usage: rgbsec-keygen zone keyfile");
        return ExitCode::from(100);
    }
    let result = arguments[0]
        .parse::<Name>()
        .map_err(|error| error.to_string())
        .and_then(|zone| {
            dnssec::generate_key(&zone, Path::new(&arguments[1])).map_err(|error| error.to_string())
        });
    match result {
        Ok(policy) => {
            println!("{}", policy.line());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("rgbsec-keygen: {error}");
            ExitCode::from(111)
        }
    }
}
