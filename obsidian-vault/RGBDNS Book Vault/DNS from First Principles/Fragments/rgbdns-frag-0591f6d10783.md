---
type: "code-fragment"
fragment_id: "rgbdns-frag-0591f6d10783"
source_path: "benches/dns_core.rs"
code_note: "DNS from First Principles/Code/benches/dns_core.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "report"
kind: "fn"
start_line: 29
end_line: 32
---

# report

- Fragment ID: `rgbdns-frag-0591f6d10783`
- Source file: [[DNS from First Principles/Code/benches/dns_core.rs.source|benches/dns_core.rs]]
- Lines: 29-32
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-0591f6d10783", "codeNote": "DNS from First Principles/Code/benches/dns_core.rs.source", "heading": "rgbdns-frag-0591f6d10783: fn report", "sourcePath": "benches/dns_core.rs", "startLine": 29, "endLine": 32}
```

## Excerpt

<span id="rgbdns-frag-0591f6d10783" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0591f6d10783: fn report

```rust
fn report(name: &str, operation: impl FnMut()) {
    println!("{name:34} {:>10} ns/op", measure(operation).as_nanos());
}

```
