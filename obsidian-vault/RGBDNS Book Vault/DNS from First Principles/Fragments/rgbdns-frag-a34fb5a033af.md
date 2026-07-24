---
type: "code-fragment"
fragment_id: "rgbdns-frag-a34fb5a033af"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "record_metadata"
kind: "fn"
start_line: 665
end_line: 682
---

# record_metadata

- Fragment ID: `rgbdns-frag-a34fb5a033af`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 665-682
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a34fb5a033af", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-a34fb5a033af: fn record_metadata", "sourcePath": "src/zone.rs", "startLine": 665, "endLine": 682}
```

## Excerpt

<span id="rgbdns-frag-a34fb5a033af" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a34fb5a033af: fn record_metadata

```rust
fn record_metadata(fields: &[String], timestamp: usize, location: usize) -> RecordMetadata {
    let text = field_opt(fields, timestamp).unwrap_or_default().as_bytes();
    let mut bytes = [0; 8];
    for (index, byte) in text.iter().take(16).enumerate() {
        let nibble = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            _ => 0,
        };
        bytes[index / 2] |= if index % 2 == 0 { nibble << 4 } else { nibble };
    }
    let location = location_code(field_opt(fields, location).unwrap_or_default());
    RecordMetadata {
        cutoff: u64::from_be_bytes(bytes),
        location: (location != [0, 0]).then_some(location),
    }
}

```
