---
type: "code-fragment"
fragment_id: "rgbdns-frag-20811dbcff5c"
source_path: "src/bin/dnsq.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsq.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsq"
symbol: "run"
kind: "fn"
start_line: 10
end_line: 24
---

# run

- Fragment ID: `rgbdns-frag-20811dbcff5c`
- Source file: [[DNS from First Principles/Code/src/bin/dnsq.rs.source|src/bin/dnsq.rs]]
- Lines: 10-24
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsq|dnsq]]

```rgbdns-fragment
{"id": "rgbdns-frag-20811dbcff5c", "codeNote": "DNS from First Principles/Code/src/bin/dnsq.rs.source", "heading": "rgbdns-frag-20811dbcff5c: fn run", "sourcePath": "src/bin/dnsq.rs", "startLine": 10, "endLine": 24}
```

## Excerpt

<span id="rgbdns-frag-20811dbcff5c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-20811dbcff5c: fn run

```rust
fn run() -> rgbdns::Result<()> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 3 {
        return Err(rgbdns::Error::Format("usage: dnsq type name server"));
    }
    let server = client::server_address(&arguments[2])?;
    let response = client::query(
        arguments[1].parse()?,
        arguments[0].parse::<RecordType>()?,
        false,
        &[server],
    )?;
    println!("{response:#?}");
    Ok(())
}
```
