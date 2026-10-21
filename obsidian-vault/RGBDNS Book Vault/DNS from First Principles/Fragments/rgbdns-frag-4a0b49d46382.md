---
type: "code-fragment"
fragment_id: "rgbdns-frag-4a0b49d46382"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "fmt"
kind: "fn"
start_line: 51
end_line: 60
---

# fmt

- Fragment ID: `rgbdns-frag-4a0b49d46382`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 51-60
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4a0b49d46382", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 51, "endLine": 60}
```

## Excerpt

<span id="rgbdns-frag-4a0b49d46382" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4a0b49d46382: fn fmt

```rust
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}\t{}\t{}\t{}\t{}\tok",
            self.zone, self.serial, self.key_tag, self.earliest_expiration, self.remaining
        )
    }
}

#[derive(Clone, Default)]
```
