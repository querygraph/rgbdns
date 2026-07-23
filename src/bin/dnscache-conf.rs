fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = rgbdns::conf::configure(rgbdns::conf::Service::Dnscache, &arguments) {
        eprintln!("dnscache-conf: fatal: {error}");
        std::process::exit(111);
    }
}
