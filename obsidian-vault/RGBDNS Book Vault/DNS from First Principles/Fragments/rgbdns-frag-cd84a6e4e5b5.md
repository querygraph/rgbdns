---
type: "code-fragment"
fragment_id: "rgbdns-frag-cd84a6e4e5b5"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "localize"
kind: "fn"
start_line: 114
end_line: 152
---

# localize

- Fragment ID: `rgbdns-frag-cd84a6e4e5b5`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 114-152
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-cd84a6e4e5b5", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-cd84a6e4e5b5: fn localize", "sourcePath": "src/tai64.rs", "startLine": 114, "endLine": 152}
```

## Excerpt

<span id="rgbdns-frag-cd84a6e4e5b5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cd84a6e4e5b5: fn localize

```rust
pub fn localize<R: BufRead, W: Write>(mut input: R, mut output: W) -> io::Result<()> {
    let mut prefix = Vec::with_capacity(25);
    loop {
        prefix.clear();
        while prefix.len() < 25 {
            let available = input.fill_buf()?;
            if available.is_empty() {
                break;
            }
            let remaining = 25 - prefix.len();
            let length = available
                .iter()
                .take(remaining)
                .position(|byte| *byte == b'\n')
                .map_or(available.len().min(remaining), |position| position + 1);
            prefix.extend_from_slice(&available[..length]);
            input.consume(length);
            if prefix.last() == Some(&b'\n') {
                break;
            }
        }
        if prefix.is_empty() {
            break;
        }
        if prefix.len() == 25
            && let Ok(value) = std::str::from_utf8(&prefix)
            && let Some(timestamp) = parse_label(value).and_then(local)
        {
            output.write_all(timestamp.as_bytes())?;
        } else {
            output.write_all(&prefix)?;
        }
        if prefix.last() != Some(&b'\n') {
            copy_line_remainder(&mut input, &mut output)?;
        }
    }
    output.flush()
}

```
