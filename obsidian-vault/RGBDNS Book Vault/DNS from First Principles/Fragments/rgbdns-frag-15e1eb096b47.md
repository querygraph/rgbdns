---
type: "code-fragment"
fragment_id: "rgbdns-frag-15e1eb096b47"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "tinydns_name"
kind: "fn"
start_line: 473
end_line: 492
---

# tinydns_name

- Fragment ID: `rgbdns-frag-15e1eb096b47`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 473-492
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-15e1eb096b47", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 473, "endLine": 492}
```

## Excerpt

<span id="rgbdns-frag-15e1eb096b47" class="rgbdns-fragment-target"></span>
### rgbdns-frag-15e1eb096b47: fn tinydns_name

```rust
fn tinydns_name(name: &Name) -> String {
    if name.is_root() {
        return ".".into();
    }
    let mut output = String::new();
    for (index, label) in name.labels().enumerate() {
        if index != 0 {
            output.push('.');
        }
        for byte in label {
            if (b'!'..=b'~').contains(byte) && !matches!(byte, b'.' | b':' | b'\\') {
                output.push(char::from(*byte));
            } else {
                output.push_str(&format!("\\{byte:03o}"));
            }
        }
    }
    output
}

```
