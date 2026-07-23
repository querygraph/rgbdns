use rgbdns::pick::{self, Database};

fn main() {
    let result =
        Database::from_file("data").and_then(|database| pick::compile(&database, "data.cdb"));
    if let Err(error) = result {
        eprintln!("pickdns-data: fatal: {error}");
        std::process::exit(111);
    }
}
