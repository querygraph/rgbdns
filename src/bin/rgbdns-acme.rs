use rgbdns::{
    Name,
    acme::{self, AdminAction},
};
use std::{net::SocketAddr, str::FromStr};

fn usage() -> ! {
    eprintln!(
        "usage:\n  rgbdns-acme present --zone Z --name N --value V [options]\n  rgbdns-acme cleanup --zone Z --name N --value V [options]\n  rgbdns-acme clear --zone Z --name N [options]\n  rgbdns-acme list [--zone Z] [options]\n\noptions:\n  --config FILE     default /etc/rgbdns/acme-update.conf\n  --server ADDRESS  default 127.0.0.1:53\n  --state-dir DIR   default /var/lib/rgbdns/tinydns\n  --json"
    );
    std::process::exit(2)
}

fn main() {
    if let Err(error) = run() {
        eprintln!("rgbdns-acme: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> rgbdns::Result<()> {
    let mut arguments = std::env::args().skip(1);
    let command = arguments.next().unwrap_or_else(|| usage());
    let mut zone = None;
    let mut owner = None;
    let mut value = None;
    let mut config = String::from("/etc/rgbdns/acme-update.conf");
    let mut server = String::from("127.0.0.1:53");
    let mut state_dir = String::from("/var/lib/rgbdns/tinydns");
    let mut json = false;
    while let Some(option) = arguments.next() {
        match option.as_str() {
            "--zone" => zone = arguments.next(),
            "--name" => owner = arguments.next(),
            "--value" => value = arguments.next(),
            "--config" => config = arguments.next().unwrap_or_else(|| usage()),
            "--server" => server = arguments.next().unwrap_or_else(|| usage()),
            "--state-dir" => state_dir = arguments.next().unwrap_or_else(|| usage()),
            "--json" => json = true,
            "-h" | "--help" => usage(),
            _ => usage(),
        }
    }
    if command == "list" {
        let zone = zone.map(|value| Name::from_str(&value)).transpose()?;
        let records = acme::list_overlay(state_dir)?;
        if json {
            print!("[");
        }
        let mut first = true;
        for (name, value) in records
            .into_iter()
            .filter(|(name, _)| zone.as_ref().is_none_or(|zone| name.is_subdomain_of(zone)))
        {
            let value = String::from_utf8_lossy(&value);
            if json {
                if !first {
                    print!(",");
                }
                print!(
                    "{{\"name\":\"{}\",\"value\":\"{}\"}}",
                    name,
                    json_escape(&value)
                );
            } else {
                println!("{name}\t{value}");
            }
            first = false;
        }
        if json {
            println!("]");
        }
        return Ok(());
    }
    let zone = Name::from_str(&zone.unwrap_or_else(|| usage()))?;
    let owner = Name::from_str(&owner.unwrap_or_else(|| usage()))?;
    let address = SocketAddr::from_str(&server)
        .map_err(|_| rgbdns::Error::Format("invalid server address"))?;
    let action = match command.as_str() {
        "present" => AdminAction::Present(value.as_deref().unwrap_or_else(|| usage()).as_bytes()),
        "cleanup" => AdminAction::Cleanup(value.as_deref().unwrap_or_else(|| usage()).as_bytes()),
        "clear" if value.is_none() => AdminAction::Clear,
        _ => usage(),
    };
    acme::admin_update(config, address, &zone, &owner, action)?;
    if json {
        println!("{{\"status\":\"ok\"}}");
    }
    Ok(())
}

fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
