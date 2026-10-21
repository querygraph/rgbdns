---
type: "code-fragment"
fragment_id: "rgbdns-frag-c97b977395ea"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "bytes"
kind: "fn"
start_line: 928
end_line: 935
---

# bytes

- Fragment ID: `rgbdns-frag-c97b977395ea`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 928-935
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c97b977395ea", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 928, "endLine": 935}
```

## Excerpt

<span id="rgbdns-frag-c97b977395ea" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c97b977395ea: fn bytes

```rust
    fn bytes(&mut self, length: usize) -> Result<&'a [u8]> {
        let value = self
            .wire
            .get(self.position..self.position + length)
            .ok_or(Error::Format("truncated UPDATE field"))?;
        self.position += length;
        Ok(value)
    }
```
