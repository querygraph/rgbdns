---
type: "code-fragment"
fragment_id: "rgbdns-frag-c9c29728a270"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "unix_now"
kind: "fn"
start_line: 952
end_line: 957
---

# unix_now

- Fragment ID: `rgbdns-frag-c9c29728a270`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 952-957
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c9c29728a270", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 952, "endLine": 957}
```

## Excerpt

<span id="rgbdns-frag-c9c29728a270" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c9c29728a270: fn unix_now

```rust
fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}
#[derive(Clone, Debug)]
```
