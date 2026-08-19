use rgbdns::dnssec;
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 2 {
        eprintln!("usage: aname-materialize [data [data.materialized]]");
        return ExitCode::from(100);
    }
    let input = Path::new(arguments.first().map_or("data", String::as_str));
    let output = Path::new(arguments.get(1).map_or("data.materialized", String::as_str));
    match dnssec::materialize_file(input, output) {
        Ok(Some(expires)) => {
            println!("{expires}\tok");
            ExitCode::SUCCESS
        }
        Ok(None) => {
            println!("-\tok");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("aname-materialize: {error}");
            ExitCode::from(111)
        }
    }
}
