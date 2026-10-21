---
type: "code-fragment"
fragment_id: "rgbdns-frag-e9082a008832"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "name"
kind: "fn"
start_line: 936
end_line: 978
---

# name

- Fragment ID: `rgbdns-frag-e9082a008832`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 936-978
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e9082a008832", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 936, "endLine": 978}
```

## Excerpt

<span id="rgbdns-frag-e9082a008832" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e9082a008832: fn name

```rust
    fn name(&mut self) -> Result<Name> {
        let mut labels = Vec::new();
        let mut cursor = self.position;
        let mut returned = None;
        let mut hops = 0;
        loop {
            hops += 1;
            if hops > 128 {
                return Err(Error::Format("UPDATE name compression loop"));
            }
            let length = *self
                .wire
                .get(cursor)
                .ok_or(Error::Format("truncated UPDATE name"))?;
            if length & 0xc0 == 0xc0 {
                let next = *self
                    .wire
                    .get(cursor + 1)
                    .ok_or(Error::Format("truncated UPDATE pointer"))?;
                let target = (usize::from(length & 0x3f) << 8) | usize::from(next);
                if target >= cursor {
                    return Err(Error::Format("forward UPDATE pointer"));
                }
                returned.get_or_insert(cursor + 2);
                cursor = target;
                continue;
            }
            if length & 0xc0 != 0 {
                return Err(Error::Format("invalid UPDATE label"));
            }
            cursor += 1;
            if length == 0 {
                self.position = returned.unwrap_or(cursor);
                return Name::from_labels(labels);
            }
            let label = self
                .wire
                .get(cursor..cursor + usize::from(length))
                .ok_or(Error::Format("truncated UPDATE label"))?;
            labels.push(label.to_vec());
            cursor += usize::from(length);
        }
    }
```
