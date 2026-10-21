---
type: "code-fragment"
fragment_id: "rgbdns-frag-37e0a52d0465"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "Zone"
kind: "impl"
start_line: 45
end_line: 47
---

# Zone

- Fragment ID: `rgbdns-frag-37e0a52d0465`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 45-47
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-37e0a52d0465", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "sourcePath": "src/zone.rs", "startLine": 45, "endLine": 47}
```

## Excerpt

<span id="rgbdns-frag-37e0a52d0465" class="rgbdns-fragment-target"></span>
### rgbdns-frag-37e0a52d0465: impl Zone

```rust
impl Zone {
    /// Returns a validated snapshot with the supplied ACME TXT overlay and SOA
    /// serial overrides. The source zone is not modified.
```
