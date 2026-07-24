---
type: "code-fragment"
fragment_id: "rgbdns-frag-39f8c406f2bb"
source_path: "src/bin/dnsipq.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsipq.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsipq"
symbol: "run"
kind: "fn"
start_line: 10
end_line: 28
---

# run

- Fragment ID: `rgbdns-frag-39f8c406f2bb`
- Source file: [[DNS from First Principles/Code/src/bin/dnsipq.rs.source|src/bin/dnsipq.rs]]
- Lines: 10-28
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsipq|dnsipq]]

```rgbdns-fragment
{"id": "rgbdns-frag-39f8c406f2bb", "codeNote": "DNS from First Principles/Code/src/bin/dnsipq.rs.source", "heading": "rgbdns-frag-39f8c406f2bb: fn run", "sourcePath": "src/bin/dnsipq.rs", "startLine": 10, "endLine": 28}
```

## Excerpt

<span id="rgbdns-frag-39f8c406f2bb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-39f8c406f2bb: fn run

```rust
fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let name: Name = argument.parse()?;
        let response = client::recursive(name.clone(), RecordType::A)?;
        print!("{name} ");
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
