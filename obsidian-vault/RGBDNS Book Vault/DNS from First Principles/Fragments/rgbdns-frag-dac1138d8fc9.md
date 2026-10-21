---
type: "code-fragment"
fragment_id: "rgbdns-frag-dac1138d8fc9"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "delay_for"
kind: "fn"
start_line: 68
end_line: 71
---

# delay_for

- Fragment ID: `rgbdns-frag-dac1138d8fc9`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 68-71
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-dac1138d8fc9", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 68, "endLine": 71}
```

## Excerpt

<span id="rgbdns-frag-dac1138d8fc9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dac1138d8fc9: fn delay_for

```rust
    async fn delay_for(duration: Duration) {
        TokioTime::delay_for(duration).await;
    }

```
