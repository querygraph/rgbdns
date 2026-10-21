---
type: "code-fragment"
fragment_id: "rgbdns-frag-07ba99154433"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "materialize_file"
kind: "fn"
start_line: 613
end_line: 616
---

# materialize_file

- Fragment ID: `rgbdns-frag-07ba99154433`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 613-616
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-07ba99154433", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 613, "endLine": 616}
```

## Excerpt

<span id="rgbdns-frag-07ba99154433" class="rgbdns-fragment-target"></span>
### rgbdns-frag-07ba99154433: fn materialize_file

```rust
pub fn materialize_file(input: &Path, output: &Path) -> Result<Option<u64>> {
    materialize_file_with_policies(input, None, output)
}

```
