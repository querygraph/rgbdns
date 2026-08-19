use rgbdns::acme;
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 4 {
        eprintln!("usage: acme-materialize data acme-config state-directory output");
        return ExitCode::from(100);
    }
    match acme::materialize_state(
        Path::new(&arguments[0]),
        Path::new(&arguments[1]),
        Path::new(&arguments[2]),
        Path::new(&arguments[3]),
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("acme-materialize: {error}");
            ExitCode::from(111)
        }
    }
}
