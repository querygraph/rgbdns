use rgbdns::{cdb, zone::Zone};
fn main() {
    let result = Zone::from_file("data").and_then(|zone| cdb::compile(&zone, "data.cdb"));
    if let Err(e) = result {
        eprintln!("tinydns-data: fatal: {e}");
        std::process::exit(111)
    }
}
