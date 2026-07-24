---
type: "code-fragment"
fragment_id: "rgbdns-frag-3c37e742efce"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "from_str"
kind: "fn"
start_line: 131
end_line: 181
---

# from_str

- Fragment ID: `rgbdns-frag-3c37e742efce`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 131-181
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3c37e742efce", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-3c37e742efce: fn from_str", "sourcePath": "src/name.rs", "startLine": 131, "endLine": 181}
```

## Excerpt

<span id="rgbdns-frag-3c37e742efce" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3c37e742efce: fn from_str

```rust
    fn from_str(input: &str) -> Result<Self> {
        if input.is_empty() || input == "." {
            return Ok(Self::root());
        }
        let s = input.strip_suffix('.').unwrap_or(input);
        let mut labels = Vec::new();
        let mut label = Vec::new();
        let bytes = s.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'.' => {
                    if label.is_empty() {
                        return Err(Error::InvalidName(input.into()));
                    }
                    labels.push(std::mem::take(&mut label));
                    i += 1;
                }
                b'\\' => {
                    i += 1;
                    if i == bytes.len() {
                        return Err(Error::InvalidName(input.into()));
                    }
                    if i + 2 < bytes.len() && bytes[i..i + 3].iter().all(u8::is_ascii_digit) {
                        let n = (bytes[i] - b'0') as u16 * 100
                            + (bytes[i + 1] - b'0') as u16 * 10
                            + (bytes[i + 2] - b'0') as u16;
                        if n > 255 {
                            return Err(Error::InvalidName(input.into()));
                        }
                        label.push(n as u8);
                        i += 3;
                    } else {
                        label.push(bytes[i]);
                        i += 1;
                    }
                }
                c => {
                    label.push(c);
                    i += 1;
                }
            }
        }
        if label.is_empty() {
            return Err(Error::InvalidName(input.into()));
        }
        labels.push(label);
        Self::from_labels(labels)
    }
}

```
