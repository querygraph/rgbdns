---
type: "code-fragment"
fragment_id: "rgbdns-frag-24f0c6bf421c"
source_path: "src/bin/axfrdns.rs"
code_note: "DNS from First Principles/Code/src/bin/axfrdns.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "axfrdns"
symbol: "run"
kind: "fn"
start_line: 11
end_line: 25
---

# run

- Fragment ID: `rgbdns-frag-24f0c6bf421c`
- Source file: [[DNS from First Principles/Code/src/bin/axfrdns.rs.source|src/bin/axfrdns.rs]]
- Lines: 11-25
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/axfrdns|axfrdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-24f0c6bf421c", "codeNote": "DNS from First Principles/Code/src/bin/axfrdns.rs.source", "heading": "rgbdns-frag-24f0c6bf421c: fn run", "sourcePath": "src/bin/axfrdns.rs", "startLine": 11, "endLine": 25}
```

## Excerpt

<span id="rgbdns-frag-24f0c6bf421c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-24f0c6bf421c: fn run

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::env::var("DATA").unwrap_or_else(|_| "data.cdb".into());
    let ip = std::env::var("IP").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "53".into());
    let address = rgbdns::socket_address(&ip, &port)?;
    let allowed = std::env::var("ALLOW_NETS")
        .unwrap_or_else(|_| "127.0.0.0/8,::1/128".into())
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::parse::<IpNet>)
        .collect::<Result<Vec<_>, _>>()?;
    axfr::serve(Zone::from_file(data)?, &address.to_string(), allowed)?;
    Ok(())
}
```
