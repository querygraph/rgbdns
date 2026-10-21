---
type: "code-fragment"
fragment_id: "rgbdns-frag-e49a6dc9a80d"
source_path: "src/dnssec.rs"
code_note: "DNS from First Principles/Code/src/dnssec.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "line"
kind: "fn"
start_line: 235
end_line: 247
---

# line

- Fragment ID: `rgbdns-frag-e49a6dc9a80d`
- Source file: [[DNS from First Principles/Code/src/dnssec.rs.source|src/dnssec.rs]]
- Lines: 235-247
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e49a6dc9a80d", "codeNote": "DNS from First Principles/Code/src/dnssec.rs.source", "sourcePath": "src/dnssec.rs", "startLine": 235, "endLine": 247}
```

## Excerpt

<span id="rgbdns-frag-e49a6dc9a80d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e49a6dc9a80d: fn line

```rust
    pub fn line(&self) -> String {
        format!(
            "K{}:{}:{}:{}:{}:{}",
            self.zone,
            self.key_file.display(),
            ALGORITHM,
            self.validity,
            self.refresh,
            self.inception_skew
        )
    }
}

```
