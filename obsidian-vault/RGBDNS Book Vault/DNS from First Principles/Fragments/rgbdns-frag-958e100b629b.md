---
type: "code-fragment"
fragment_id: "rgbdns-frag-958e100b629b"
source_path: "src/pick.rs"
code_note: "DNS from First Principles/Code/src/pick.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "bad_line"
kind: "fn"
start_line: 206
end_line: 209
---

# bad_line

- Fragment ID: `rgbdns-frag-958e100b629b`
- Source file: [[DNS from First Principles/Code/src/pick.rs.source|src/pick.rs]]
- Lines: 206-209
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-958e100b629b", "codeNote": "DNS from First Principles/Code/src/pick.rs.source", "sourcePath": "src/pick.rs", "startLine": 206, "endLine": 209}
```

## Excerpt

<span id="rgbdns-frag-958e100b629b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-958e100b629b: fn bad_line

```rust
fn bad_line(number: usize, message: &str) -> Error {
    Error::InvalidRecord(format!("line {}: {message}", number + 1))
}

```
