use rgbdns::dnssec;
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 2 {
        eprintln!("usage: dnssec-data [data [data.cdb]]");
        return ExitCode::from(100);
    }
    let input = Path::new(arguments.first().map_or("data", String::as_str));
    let output = Path::new(arguments.get(1).map_or("data.cdb", String::as_str));
    match dnssec::compile_file(input, Path::new("dnssec"), output) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("dnssec-data: {error}");
            ExitCode::from(111)
        }
    }
}
