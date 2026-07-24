---
type: "code-fragment"
fragment_id: "rgbdns-frag-04dab64f1b7a"
source_path: "src/bin/random-ip.rs"
code_note: "DNS from First Principles/Code/src/bin/random-ip.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "random-ip"
symbol: "uniform"
kind: "fn"
start_line: 62
end_line: 72
---

# uniform

- Fragment ID: `rgbdns-frag-04dab64f1b7a`
- Source file: [[DNS from First Principles/Code/src/bin/random-ip.rs.source|src/bin/random-ip.rs]]
- Lines: 62-72
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/random-ip|random-ip]]

```rgbdns-fragment
{"id": "rgbdns-frag-04dab64f1b7a", "codeNote": "DNS from First Principles/Code/src/bin/random-ip.rs.source", "heading": "rgbdns-frag-04dab64f1b7a: fn uniform", "sourcePath": "src/bin/random-ip.rs", "startLine": 62, "endLine": 72}
```

## Excerpt

<span id="rgbdns-frag-04dab64f1b7a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-04dab64f1b7a: fn uniform

```rust
fn uniform(upper: u32) -> Result<u32, getrandom::Error> {
    let threshold = u32::MAX - (u32::MAX % upper);
    loop {
        let mut bytes = [0; 4];
        getrandom::fill(&mut bytes)?;
        let value = u32::from_ne_bytes(bytes);
        if value < threshold {
            return Ok(value % upper);
        }
    }
}
```
