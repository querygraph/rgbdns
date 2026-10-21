---
type: "code-fragment"
fragment_id: "rgbdns-frag-9fb33291dffb"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "current_time"
kind: "fn"
start_line: 79
end_line: 86
---

# current_time

- Fragment ID: `rgbdns-frag-9fb33291dffb`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 79-86
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9fb33291dffb", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 79, "endLine": 86}
```

## Excerpt

<span id="rgbdns-frag-9fb33291dffb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9fb33291dffb: fn current_time

```rust
    fn current_time() -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_secs());
        SIGNING_SKEW.with(|skew| now.saturating_sub(skew.get()))
    }
}

```
