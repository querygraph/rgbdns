---
type: "code-fragment"
fragment_id: "rgbdns-frag-68fc31fecca6"
source_path: "src/bin/dnsmx.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsmx.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsmx"
symbol: "run"
kind: "fn"
start_line: 10
end_line: 32
---

# run

- Fragment ID: `rgbdns-frag-68fc31fecca6`
- Source file: [[DNS from First Principles/Code/src/bin/dnsmx.rs.source|src/bin/dnsmx.rs]]
- Lines: 10-32
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsmx|dnsmx]]

```rgbdns-fragment
{"id": "rgbdns-frag-68fc31fecca6", "codeNote": "DNS from First Principles/Code/src/bin/dnsmx.rs.source", "heading": "rgbdns-frag-68fc31fecca6: fn run", "sourcePath": "src/bin/dnsmx.rs", "startLine": 10, "endLine": 32}
```

## Excerpt

<span id="rgbdns-frag-68fc31fecca6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-68fc31fecca6: fn run

```rust
fn run() -> rgbdns::Result<()> {
    for argument in std::env::args().skip(1) {
        let name = argument.parse()?;
        let response = client::recursive(name, RecordType::Mx)?;
        let mut found = false;
        for (preference, target) in
            response
                .answers
                .iter()
                .filter_map(|record| match &record.data {
                    RData::Mx(preference, target) => Some((preference, target)),
                    _ => None,
                })
        {
            println!("{preference} {target}");
            found = true;
        }
        if !found {
            println!("0 {argument}.");
        }
    }
    Ok(())
}
```
