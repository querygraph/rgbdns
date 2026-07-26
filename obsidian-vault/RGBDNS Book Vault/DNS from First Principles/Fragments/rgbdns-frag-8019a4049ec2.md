---
type: "code-fragment"
fragment_id: "rgbdns-frag-8019a4049ec2"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "unescape"
kind: "fn"
start_line: 760
end_line: 794
---

# unescape

- Fragment ID: `rgbdns-frag-8019a4049ec2`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 760-794
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-8019a4049ec2", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-8019a4049ec2: fn unescape", "sourcePath": "src/zone.rs", "startLine": 760, "endLine": 794}
```

## Excerpt

<span id="rgbdns-frag-8019a4049ec2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8019a4049ec2: fn unescape

```rust
fn unescape(s: &str) -> Result<Vec<u8>> {
    let mut o = Vec::new();
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' {
            i += 1;
            if i >= b.len() {
                return Err(Error::InvalidRecord("trailing escape".into()));
            }
            if (b'0'..=b'7').contains(&b[i]) {
                let mut n = b[i] - b'0';
                i += 1;
                for _ in 0..2 {
                    if i < b.len() && (b'0'..=b'7').contains(&b[i]) {
                        n = n.wrapping_mul(8).wrapping_add(b[i] - b'0');
                        i += 1;
                    } else {
                        break;
                    }
                }
                o.push(n)
            } else {
                o.push(b[i]);
                i += 1
            }
        } else {
            o.push(b[i]);
            i += 1
        }
    }
    Ok(o)
}

#[cfg(test)]
```
