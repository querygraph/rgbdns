---
type: "code-fragment"
fragment_id: "rgbdns-frag-7a0c14c0646b"
source_path: "src/bin/dnsip6q.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsip6q.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsip6q"
symbol: "run"
kind: "fn"
start_line: 10
end_line: 28
---

# run

- Fragment ID: `rgbdns-frag-7a0c14c0646b`
- Source file: [[DNS from First Principles/Code/src/bin/dnsip6q.rs.source|src/bin/dnsip6q.rs]]
- Lines: 10-28
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsip6q|dnsip6q]]

```rgbdns-fragment
{"id": "rgbdns-frag-7a0c14c0646b", "codeNote": "DNS from First Principles/Code/src/bin/dnsip6q.rs.source", "heading": "rgbdns-frag-7a0c14c0646b: fn run", "sourcePath": "src/bin/dnsip6q.rs", "startLine": 10, "endLine": 28}
```

## Excerpt

<span id="rgbdns-frag-7a0c14c0646b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7a0c14c0646b: fn run

```rust
fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let name: Name = argument.parse()?;
        let response = client::recursive(name.clone(), RecordType::Aaaa)?;
        print!("{name} ");
        for address in response
            .answers
            .iter()
            .filter_map(|record| match record.data {
                RData::Aaaa(address) => Some(address),
                _ => None,
            })
        {
            print!("{address} ");
        }
        println!();
    }
    Ok(())
}
```
