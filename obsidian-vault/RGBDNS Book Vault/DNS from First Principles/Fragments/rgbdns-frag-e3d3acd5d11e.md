---
type: "code-fragment"
fragment_id: "rgbdns-frag-e3d3acd5d11e"
source_path: "src/bin/dnstxt.rs"
code_note: "DNS from First Principles/Code/src/bin/dnstxt.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnstxt"
symbol: "run"
kind: "fn"
start_line: 11
end_line: 28
---

# run

- Fragment ID: `rgbdns-frag-e3d3acd5d11e`
- Source file: [[DNS from First Principles/Code/src/bin/dnstxt.rs.source|src/bin/dnstxt.rs]]
- Lines: 11-28
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnstxt|dnstxt]]

```rgbdns-fragment
{"id": "rgbdns-frag-e3d3acd5d11e", "codeNote": "DNS from First Principles/Code/src/bin/dnstxt.rs.source", "heading": "rgbdns-frag-e3d3acd5d11e: fn run", "sourcePath": "src/bin/dnstxt.rs", "startLine": 11, "endLine": 28}
```

## Excerpt

<span id="rgbdns-frag-e3d3acd5d11e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e3d3acd5d11e: fn run

```rust
fn run() -> rgbdns::Result<()> {
    let mut stdout = std::io::stdout().lock();
    for argument in std::env::args().skip(1) {
        let response = client::recursive(argument.parse()?, RecordType::Txt)?;
        for chunk in response
            .answers
            .iter()
            .flat_map(|record| match &record.data {
                RData::Txt(chunks) => chunks.as_slice(),
                _ => &[],
            })
        {
            stdout.write_all(chunk)?;
        }
        stdout.write_all(b"\n")?;
    }
    Ok(())
}
```
