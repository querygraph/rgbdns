---
type: "code-fragment"
fragment_id: "rgbdns-frag-47a1aedc6982"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "stamp"
kind: "fn"
start_line: 91
end_line: 113
---

# stamp

- Fragment ID: `rgbdns-frag-47a1aedc6982`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 91-113
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-47a1aedc6982", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-47a1aedc6982: fn stamp", "sourcePath": "src/tai64.rs", "startLine": 91, "endLine": 113}
```

## Excerpt

<span id="rgbdns-frag-47a1aedc6982" class="rgbdns-fragment-target"></span>
### rgbdns-frag-47a1aedc6982: fn stamp

```rust
pub fn stamp<R: BufRead, W: Write>(mut input: R, mut output: W) -> io::Result<()> {
    let mut line_start = true;
    loop {
        let available = input.fill_buf()?;
        if available.is_empty() {
            break;
        }
        let length = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |position| position + 1);
        let segment = &available[..length];
        if line_start {
            output.write_all(label(SystemTime::now()).as_bytes())?;
            output.write_all(b" ")?;
        }
        output.write_all(segment)?;
        line_start = segment.last() == Some(&b'\n');
        input.consume(length);
    }
    output.flush()
}

```
