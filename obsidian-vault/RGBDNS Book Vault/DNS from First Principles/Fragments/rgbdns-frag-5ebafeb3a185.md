---
type: "code-fragment"
fragment_id: "rgbdns-frag-5ebafeb3a185"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "write"
kind: "fn"
start_line: 134
end_line: 137
---

# write

- Fragment ID: `rgbdns-frag-5ebafeb3a185`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 134-137
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5ebafeb3a185", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-5ebafeb3a185: fn write", "sourcePath": "src/multilog.rs", "startLine": 134, "endLine": 137}
```

## Excerpt

<span id="rgbdns-frag-5ebafeb3a185" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5ebafeb3a185: fn write

```rust
    fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.write_parts(bytes, &[])
    }

```
