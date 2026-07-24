---
type: "code-fragment"
fragment_id: "rgbdns-frag-b143870e07fb"
source_path: "benches/dns_core.rs"
code_note: "DNS from First Principles/Code/benches/dns_core.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "DEFAULT_ITERATIONS"
kind: "const"
start_line: 8
end_line: 13
---

# DEFAULT_ITERATIONS

- Fragment ID: `rgbdns-frag-b143870e07fb`
- Source file: [[DNS from First Principles/Code/benches/dns_core.rs.source|benches/dns_core.rs]]
- Lines: 8-13
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-b143870e07fb", "codeNote": "DNS from First Principles/Code/benches/dns_core.rs.source", "heading": "rgbdns-frag-b143870e07fb: const DEFAULT_ITERATIONS", "sourcePath": "benches/dns_core.rs", "startLine": 8, "endLine": 13}
```

## Excerpt

<span id="rgbdns-frag-b143870e07fb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b143870e07fb: const DEFAULT_ITERATIONS

```rust
const DEFAULT_ITERATIONS: u64 = if cfg!(debug_assertions) {
    1_000
} else {
    100_000
};

```
