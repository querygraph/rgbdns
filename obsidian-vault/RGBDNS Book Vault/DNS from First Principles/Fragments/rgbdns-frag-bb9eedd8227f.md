---
type: "code-fragment"
fragment_id: "rgbdns-frag-bb9eedd8227f"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "from_env"
kind: "fn"
start_line: 19
end_line: 29
---

# from_env

- Fragment ID: `rgbdns-frag-bb9eedd8227f`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 19-29
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-bb9eedd8227f", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 19, "endLine": 29}
```

## Excerpt

<span id="rgbdns-frag-bb9eedd8227f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bb9eedd8227f: fn from_env

```rust
    fn from_env() -> Result<Self> {
        let enabled = match std::env::var("QUERY_LOG") {
            Ok(value) => parse_query_log(Some(&value))?,
            Err(std::env::VarError::NotPresent) => true,
            Err(std::env::VarError::NotUnicode(_)) => {
                return Err(Error::Format("invalid QUERY_LOG"));
            }
        };
        Ok(Self { enabled })
    }

```
