---
type: "code-fragment"
fragment_id: "rgbdns-frag-a2e3aa103cac"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "write_parts"
kind: "fn"
start_line: 138
end_line: 148
---

# write_parts

- Fragment ID: `rgbdns-frag-a2e3aa103cac`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 138-148
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a2e3aa103cac", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-a2e3aa103cac: fn write_parts", "sourcePath": "src/multilog.rs", "startLine": 138, "endLine": 148}
```

## Excerpt

<span id="rgbdns-frag-a2e3aa103cac" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a2e3aa103cac: fn write_parts

```rust
    fn write_parts(&mut self, first: &[u8], second: &[u8]) -> io::Result<()> {
        let added = (first.len() as u64).saturating_add(second.len() as u64);
        if self.size > 0 && self.size.saturating_add(added) > self.maximum {
            self.rotate()?;
        }
        self.file.write_all(first)?;
        self.file.write_all(second)?;
        self.size = self.size.saturating_add(added);
        Ok(())
    }

```
