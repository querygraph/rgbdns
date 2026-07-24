---
type: "code-fragment"
fragment_id: "rgbdns-frag-11cab49278dc"
source_path: "src/bin/dnsip.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsip.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsip"
symbol: "run"
kind: "fn"
start_line: 10
end_line: 26
---

# run

- Fragment ID: `rgbdns-frag-11cab49278dc`
- Source file: [[DNS from First Principles/Code/src/bin/dnsip.rs.source|src/bin/dnsip.rs]]
- Lines: 10-26
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsip|dnsip]]

```rgbdns-fragment
{"id": "rgbdns-frag-11cab49278dc", "codeNote": "DNS from First Principles/Code/src/bin/dnsip.rs.source", "heading": "rgbdns-frag-11cab49278dc: fn run", "sourcePath": "src/bin/dnsip.rs", "startLine": 10, "endLine": 26}
```

## Excerpt

<span id="rgbdns-frag-11cab49278dc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-11cab49278dc: fn run

```rust
fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let response = client::recursive(argument.parse()?, RecordType::A)?;
        for address in response
            .answers
            .iter()
            .filter_map(|record| match record.data {
                RData::A(address) => Some(address),
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
