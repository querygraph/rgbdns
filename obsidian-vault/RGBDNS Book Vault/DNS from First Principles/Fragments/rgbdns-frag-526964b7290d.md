---
type: "code-fragment"
fragment_id: "rgbdns-frag-526964b7290d"
source_path: "tests/cdb_golden.rs"
code_note: "DNS from First Principles/Code/tests/cdb_golden.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "hex"
kind: "fn"
start_line: 45
end_line: 48
---

# hex

- Fragment ID: `rgbdns-frag-526964b7290d`
- Source file: [[DNS from First Principles/Code/tests/cdb_golden.rs.source|tests/cdb_golden.rs]]
- Lines: 45-48
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-526964b7290d", "codeNote": "DNS from First Principles/Code/tests/cdb_golden.rs.source", "heading": "rgbdns-frag-526964b7290d: fn hex", "sourcePath": "tests/cdb_golden.rs", "startLine": 45, "endLine": 48}
```

## Excerpt

<span id="rgbdns-frag-526964b7290d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-526964b7290d: fn hex

```rust
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

```
