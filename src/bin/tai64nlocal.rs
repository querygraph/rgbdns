use std::io::BufReader;

fn main() {
    if std::env::args_os().len() != 1
        || rgbdns::tai64::localize(
            BufReader::new(std::io::stdin().lock()),
            std::io::stdout().lock(),
        )
        .is_err()
    {
        std::process::exit(111);
    }
}
