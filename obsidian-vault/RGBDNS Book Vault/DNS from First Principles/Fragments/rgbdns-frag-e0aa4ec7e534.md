---
type: "code-fragment"
fragment_id: "rgbdns-frag-e0aa4ec7e534"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "timeout"
kind: "fn"
start_line: 72
end_line: 78
---

# timeout

- Fragment ID: `rgbdns-frag-e0aa4ec7e534`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 72-78
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e0aa4ec7e534", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 72, "endLine": 78}
```

## Excerpt

<span id="rgbdns-frag-e0aa4ec7e534" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e0aa4ec7e534: fn timeout

```rust
    async fn timeout<F: 'static + Future + Send>(
        duration: Duration,
        future: F,
    ) -> std::result::Result<F::Output, io::Error> {
        TokioTime::timeout(duration, future).await
    }

```
