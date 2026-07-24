---
type: "code-fragment"
fragment_id: "rgbdns-frag-2f06d4f4885b"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "path"
kind: "fn"
start_line: 99
end_line: 104
---

# path

- Fragment ID: `rgbdns-frag-2f06d4f4885b`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 99-104
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2f06d4f4885b", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-2f06d4f4885b: fn path", "sourcePath": "src/dnscache_config.rs", "startLine": 99, "endLine": 104}
```

## Excerpt

<span id="rgbdns-frag-2f06d4f4885b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2f06d4f4885b: fn path

```rust
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Loads original djbdns `ROOT/servers/domain` forwarding rules.
```
