---
type: "code-fragment"
fragment_id: "rgbdns-frag-0475cb5e016a"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "validate"
kind: "fn"
start_line: 116
end_line: 128
---

# validate

- Fragment ID: `rgbdns-frag-0475cb5e016a`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 116-128
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0475cb5e016a", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-0475cb5e016a: fn validate", "sourcePath": "src/name.rs", "startLine": 116, "endLine": 128}
```

## Excerpt

<span id="rgbdns-frag-0475cb5e016a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0475cb5e016a: fn validate

```rust
fn validate(labels: &[Vec<u8>]) -> Result<()> {
    if labels.iter().any(|l| l.is_empty() || l.len() > 63) {
        return Err(Error::InvalidName(
            "label must contain 1..=63 octets".into(),
        ));
    }
    let len = 1 + labels.iter().map(|l| l.len() + 1).sum::<usize>();
    if len > 255 {
        return Err(Error::InvalidName("wire name exceeds 255 octets".into()));
    }
    Ok(())
}

```
