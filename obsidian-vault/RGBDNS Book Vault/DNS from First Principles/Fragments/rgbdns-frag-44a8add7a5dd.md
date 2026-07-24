---
type: "code-fragment"
fragment_id: "rgbdns-frag-44a8add7a5dd"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "copy_line_remainder"
kind: "fn"
start_line: 153
end_line: 171
---

# copy_line_remainder

- Fragment ID: `rgbdns-frag-44a8add7a5dd`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 153-171
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-44a8add7a5dd", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-44a8add7a5dd: fn copy_line_remainder", "sourcePath": "src/tai64.rs", "startLine": 153, "endLine": 171}
```

## Excerpt

<span id="rgbdns-frag-44a8add7a5dd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-44a8add7a5dd: fn copy_line_remainder

```rust
fn copy_line_remainder<R: BufRead, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    loop {
        let available = input.fill_buf()?;
        if available.is_empty() {
            return Ok(());
        }
        let length = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |position| position + 1);
        let line_ended = available[length - 1] == b'\n';
        output.write_all(&available[..length])?;
        input.consume(length);
        if line_ended {
            return Ok(());
        }
    }
}

```
