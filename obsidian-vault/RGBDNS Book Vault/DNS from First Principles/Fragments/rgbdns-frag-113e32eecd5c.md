---
type: "code-fragment"
fragment_id: "rgbdns-frag-113e32eecd5c"
source_path: "benches/dns_core.rs"
code_note: "DNS from First Principles/Code/benches/dns_core.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "measure"
kind: "fn"
start_line: 14
end_line: 28
---

# measure

- Fragment ID: `rgbdns-frag-113e32eecd5c`
- Source file: [[DNS from First Principles/Code/benches/dns_core.rs.source|benches/dns_core.rs]]
- Lines: 14-28
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-113e32eecd5c", "codeNote": "DNS from First Principles/Code/benches/dns_core.rs.source", "heading": "rgbdns-frag-113e32eecd5c: fn measure", "sourcePath": "benches/dns_core.rs", "startLine": 14, "endLine": 28}
```

## Excerpt

<span id="rgbdns-frag-113e32eecd5c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-113e32eecd5c: fn measure

```rust
fn measure(mut operation: impl FnMut()) -> Duration {
    let iterations = std::env::var("RGBDNS_BENCH_ITERATIONS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(DEFAULT_ITERATIONS);
    for _ in 0..iterations.min(1_000) {
        operation();
    }
    let start = Instant::now();
    for _ in 0..iterations {
        operation();
    }
    start.elapsed() / u32::try_from(iterations).unwrap()
}

```
