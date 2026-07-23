use rgbdns::{RecordType, client};
fn main() {
    let a = std::env::args().skip(1).collect::<Vec<_>>();
    if a.len() != 3 {
        eprintln!("usage: dnsq type name server");
        std::process::exit(100)
    }
    let server = client::server_address(&a[2]).unwrap_or_else(|error| {
        eprintln!("dnsq: fatal: {error}");
        std::process::exit(100)
    });
    let response = client::query(
        a[1].parse().unwrap(),
        a[0].parse::<RecordType>().unwrap(),
        false,
        &[server],
    )
    .unwrap();
    println!("{response:#?}")
}
