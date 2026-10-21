---
type: "code-fragment"
fragment_id: "rgbdns-frag-c7f726a96454"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "validate"
kind: "fn"
start_line: 128
end_line: 140
---

# validate

- Fragment ID: `rgbdns-frag-c7f726a96454`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 128-140
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c7f726a96454", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 128, "endLine": 140}
```

## Excerpt

<span id="rgbdns-frag-c7f726a96454" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c7f726a96454: fn validate

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
