---
type: "code-fragment"
fragment_id: "rgbdns-frag-2671eb38d646"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "fmt"
kind: "fn"
start_line: 195
end_line: 215
---

# fmt

- Fragment ID: `rgbdns-frag-2671eb38d646`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 195-215
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2671eb38d646", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "sourcePath": "src/name.rs", "startLine": 195, "endLine": 215}
```

## Excerpt

<span id="rgbdns-frag-2671eb38d646" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2671eb38d646: fn fmt

```rust
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return f.write_str(".");
        }
        for (i, label) in self.0.iter().enumerate() {
            if i != 0 {
                f.write_str(".")?;
            }
            for &c in label {
                match c {
                    b'.' | b'\\' => write!(f, "\\{}", c as char)?,
                    0x21..=0x7e => write!(f, "{}", c as char)?,
                    _ => write!(f, "\\{c:03}")?,
                }
            }
        }
        f.write_str(".")
    }
}

#[cfg(test)]
```
