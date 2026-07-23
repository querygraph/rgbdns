use rgbdns::rbl::{self, Database};

fn main() {
    let result =
        Database::from_file("data").and_then(|database| rbl::compile(&database, "data.cdb"));
    if let Err(error) = result {
        eprintln!("rbldns-data: fatal: {error}");
        std::process::exit(111);
    }
}
