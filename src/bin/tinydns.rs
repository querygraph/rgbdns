use rgbdns::{server, zone::Zone};
fn main() {
    let data = std::env::var("DATA").unwrap_or_else(|_| "data.cdb".into());
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    let address = rgbdns::socket_address(&ip, &port).unwrap_or_else(|e| {
        eprintln!("tinydns: fatal: {e}");
        std::process::exit(111)
    });
    let z = Zone::from_file(data).unwrap_or_else(|e| {
        eprintln!("tinydns: fatal: {e}");
        std::process::exit(111)
    });
    if let Err(e) = server::serve(z, &address.to_string()) {
        eprintln!("tinydns: fatal: {e}");
        std::process::exit(111)
    }
}
