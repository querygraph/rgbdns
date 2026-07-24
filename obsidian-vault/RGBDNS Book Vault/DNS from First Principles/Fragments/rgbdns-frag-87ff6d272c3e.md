---
type: "code-fragment"
fragment_id: "rgbdns-frag-87ff6d272c3e"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "recursive"
kind: "fn"
start_line: 13
end_line: 16
---

# recursive

- Fragment ID: `rgbdns-frag-87ff6d272c3e`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 13-16
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-87ff6d272c3e", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-87ff6d272c3e: fn recursive", "sourcePath": "src/client.rs", "startLine": 13, "endLine": 16}
```

## Excerpt

<span id="rgbdns-frag-87ff6d272c3e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-87ff6d272c3e: fn recursive

```rust
pub fn recursive(name: Name, record_type: RecordType) -> Result<Message> {
    query(name, record_type, true, &servers()?)
}

```
