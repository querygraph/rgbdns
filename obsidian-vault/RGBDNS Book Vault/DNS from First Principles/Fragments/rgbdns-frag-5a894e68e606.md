---
type: "code-fragment"
fragment_id: "rgbdns-frag-5a894e68e606"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "parse"
kind: "fn"
start_line: 23
end_line: 37
---

# parse

- Fragment ID: `rgbdns-frag-5a894e68e606`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 23-37
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5a894e68e606", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-5a894e68e606: fn parse", "sourcePath": "src/tinydns_edit.rs", "startLine": 23, "endLine": 37}
```

## Excerpt

<span id="rgbdns-frag-5a894e68e606" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5a894e68e606: fn parse

```rust
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "ns" => Ok(Self::Ns),
            "childns" => Ok(Self::ChildNs),
            "host" => Ok(Self::Host),
            "alias" => Ok(Self::Alias),
            "mx" => Ok(Self::Mx),
            "host6" => Ok(Self::Host6),
            "alias6" => Ok(Self::Alias6),
            _ => Err(Error::Format("invalid tinydns-edit mode")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
```
