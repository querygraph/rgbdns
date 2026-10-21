---
type: "code-fragment"
fragment_id: "rgbdns-frag-8f4360fc1331"
source_path: "src/bin/tinydns-data.rs"
code_note: "DNS from First Principles/Code/src/bin/tinydns-data.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-data"
symbol: "main"
kind: "fn"
start_line: 2
end_line: 19
---

# main

- Fragment ID: `rgbdns-frag-8f4360fc1331`
- Source file: [[DNS from First Principles/Code/src/bin/tinydns-data.rs.source|src/bin/tinydns-data.rs]]
- Lines: 2-19
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-data|tinydns-data]]

```rgbdns-fragment
{"id": "rgbdns-frag-8f4360fc1331", "codeNote": "DNS from First Principles/Code/src/bin/tinydns-data.rs.source", "sourcePath": "src/bin/tinydns-data.rs", "startLine": 2, "endLine": 19}
```

## Excerpt

<span id="rgbdns-frag-8f4360fc1331" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8f4360fc1331: fn main

```rust
fn main() {
    let result = Zone::from_file("data").and_then(|zone| {
        cdb::compile(&zone, "data.cdb")?;
        if let Ok(config) = std::env::var("ACME_UPDATE_CONFIG")
            && !config.trim().is_empty()
        {
            let state_dir = std::env::var("ACME_STATE_DIR")
                .unwrap_or_else(|_| "/var/lib/rgbdns/tinydns".into());
            let live = rgbdns::acme::LiveZone::new(zone.clone());
            let _ = rgbdns::acme::AcmeUpdates::from_file(config, state_dir, zone, live)?;
        }
        Ok(())
    });
    if let Err(e) = result {
        eprintln!("tinydns-data: fatal: {e}");
        std::process::exit(111)
    }
}
```
