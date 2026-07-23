use rgbdns::zone::Zone;
fn main() {
    if let Err(e) = Zone::from_file("data") {
        eprintln!("tinydns-data: fatal: {e}");
        std::process::exit(111)
    }
    println!("tinydns-data: data validated (rgbdns reads the safe text database directly)")
}
