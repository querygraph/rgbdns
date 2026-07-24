---
type: "code-fragment"
fragment_id: "rgbdns-frag-7629df224c60"
source_path: "src/bin/tinydns.rs"
code_note: "DNS from First Principles/Code/src/bin/tinydns.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "tinydns"
symbol: "main"
kind: "fn"
start_line: 2
end_line: 18
---

# main

- Fragment ID: `rgbdns-frag-7629df224c60`
- Source file: [[DNS from First Principles/Code/src/bin/tinydns.rs.source|src/bin/tinydns.rs]]
- Lines: 2-18
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/tinydns|tinydns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7629df224c60", "codeNote": "DNS from First Principles/Code/src/bin/tinydns.rs.source", "heading": "rgbdns-frag-7629df224c60: fn main", "sourcePath": "src/bin/tinydns.rs", "startLine": 2, "endLine": 18}
```

## Excerpt

<span id="rgbdns-frag-7629df224c60" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7629df224c60: fn main

```rust
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
```
