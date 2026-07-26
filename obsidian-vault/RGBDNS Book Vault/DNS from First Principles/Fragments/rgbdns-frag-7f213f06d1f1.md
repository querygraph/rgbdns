---
type: "code-fragment"
fragment_id: "rgbdns-frag-7f213f06d1f1"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "unix_now"
kind: "fn"
start_line: 684
end_line: 689
---

# unix_now

- Fragment ID: `rgbdns-frag-7f213f06d1f1`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 684-689
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7f213f06d1f1", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-7f213f06d1f1: fn unix_now", "sourcePath": "src/zone.rs", "startLine": 684, "endLine": 689}
```

## Excerpt

<span id="rgbdns-frag-7f213f06d1f1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7f213f06d1f1: fn unix_now

```rust
fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}
#[derive(Clone, Debug)]
```
