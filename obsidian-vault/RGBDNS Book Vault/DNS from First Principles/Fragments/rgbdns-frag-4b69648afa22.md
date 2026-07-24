---
type: "code-fragment"
fragment_id: "rgbdns-frag-4b69648afa22"
source_path: "src/bin/dnsqr.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsqr.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsqr"
symbol: "run"
kind: "fn"
start_line: 10
end_line: 20
---

# run

- Fragment ID: `rgbdns-frag-4b69648afa22`
- Source file: [[DNS from First Principles/Code/src/bin/dnsqr.rs.source|src/bin/dnsqr.rs]]
- Lines: 10-20
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsqr|dnsqr]]

```rgbdns-fragment
{"id": "rgbdns-frag-4b69648afa22", "codeNote": "DNS from First Principles/Code/src/bin/dnsqr.rs.source", "heading": "rgbdns-frag-4b69648afa22: fn run", "sourcePath": "src/bin/dnsqr.rs", "startLine": 10, "endLine": 20}
```

## Excerpt

<span id="rgbdns-frag-4b69648afa22" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4b69648afa22: fn run

```rust
fn run() -> rgbdns::Result<()> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 {
        return Err(rgbdns::Error::Format("usage: dnsqr type name"));
    }
    let record_type = arguments[0].parse::<RecordType>()?;
    let name = arguments[1].parse()?;
    println!("{} {}:", record_type.code(), name);
    println!("{:#?}", client::recursive(name, record_type)?);
    Ok(())
}
```
