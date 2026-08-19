use rgbdns::dnssec;
use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 1 {
        eprintln!("usage: rgbsec-ds dnssec-line");
        return ExitCode::from(100);
    }
    match dnssec::Policy::parse(&arguments[0]).and_then(|policy| dnssec::ds_line(&policy)) {
        Ok(line) => {
            println!("{line}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("rgbsec-ds: {error}");
            ExitCode::from(111)
        }
    }
}
