---
type: "code-file"
source_path: "src/bin/rgbdns-acme.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-acme"
line_count: 94
fragment_count: 4
rgbdns_commit: "5ae24772"
---

# src/bin/rgbdns-acme.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-acme|rgbdns-acme]]
- Source path: `src/bin/rgbdns-acme.rs`
- Lines: 94
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-ddfc81828cbd|usage]]: lines 7-13
- [[DNS from First Principles/Fragments/rgbdns-frag-e7609f107c68|main]]: lines 14-20
- [[DNS from First Principles/Fragments/rgbdns-frag-2c31ed9ee094|run]]: lines 21-91
- [[DNS from First Principles/Fragments/rgbdns-frag-fa72eb9e4bc2|json_escape]]: lines 92-94

## Full Source

```rust
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
```
