---
type: "code-fragment"
fragment_id: "rgbdns-frag-975a8cad85d6"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "drop"
kind: "fn"
start_line: 160
end_line: 166
---

# drop

- Fragment ID: `rgbdns-frag-975a8cad85d6`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 160-166
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-975a8cad85d6", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-975a8cad85d6: fn drop", "sourcePath": "src/dnscache_config.rs", "startLine": 160, "endLine": 166}
```

## Excerpt

<span id="rgbdns-frag-975a8cad85d6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-975a8cad85d6: fn drop

```rust
    fn drop(&mut self) {
        if self.temporary {
            let _ = fs::remove_file(&self.path);
        }
    }
}

```
