---
type: "code-fragment"
fragment_id: "rgbdns-frag-2c31ed9ee094"
source_path: "src/bin/rgbdns-acme.rs"
code_note: "DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "rgbdns-acme"
symbol: "run"
kind: "fn"
start_line: 21
end_line: 91
---

# run

- Fragment ID: `rgbdns-frag-2c31ed9ee094`
- Source file: [[DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source|src/bin/rgbdns-acme.rs]]
- Lines: 21-91
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/rgbdns-acme|rgbdns-acme]]

```rgbdns-fragment
{"id": "rgbdns-frag-2c31ed9ee094", "codeNote": "DNS from First Principles/Code/src/bin/rgbdns-acme.rs.source", "sourcePath": "src/bin/rgbdns-acme.rs", "startLine": 21, "endLine": 91}
```

## Excerpt

<span id="rgbdns-frag-2c31ed9ee094" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2c31ed9ee094: fn run

```rust
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

```
