---
type: "code-fragment"
fragment_id: "rgbdns-frag-9b0af663cc6d"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "write_log_line"
kind: "fn"
start_line: 61
end_line: 66
---

# write_log_line

- Fragment ID: `rgbdns-frag-9b0af663cc6d`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 61-66
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9b0af663cc6d", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 61, "endLine": 66}
```

## Excerpt

<span id="rgbdns-frag-9b0af663cc6d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9b0af663cc6d: fn write_log_line

```rust
fn write_log_line(line: &str) {
    let stderr = std::io::stderr();
    let mut stderr = stderr.lock();
    let _ = writeln!(stderr, "{line}");
}

```
