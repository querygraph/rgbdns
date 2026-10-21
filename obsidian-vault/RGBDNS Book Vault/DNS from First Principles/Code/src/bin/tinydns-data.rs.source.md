---
type: "code-file"
source_path: "src/bin/tinydns-data.rs"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns-data"
line_count: 19
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# src/bin/tinydns-data.rs

- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns-data|tinydns-data]]
- Source path: `src/bin/tinydns-data.rs`
- Lines: 19
- Summary: Source file in the Command-line programs subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-8f4360fc1331|main]]: lines 2-19

## Full Source

```rust
use rgbdns::{cdb, zone::Zone};
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
