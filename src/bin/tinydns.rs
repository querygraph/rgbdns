use rgbdns::{server, zone::Zone};
fn main() {
    let data = std::env::var("DATA").unwrap_or_else(|_| "data".into());
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    let z = Zone::from_file(data).unwrap_or_else(|e| {
        eprintln!("tinydns: fatal: {e}");
        std::process::exit(111)
    });
    if let Err(e) = server::serve(z, &format!("{ip}:{port}")) {
        eprintln!("tinydns: fatal: {e}");
        std::process::exit(111)
    }
}
