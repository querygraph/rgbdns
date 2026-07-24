---
type: "code-fragment"
fragment_id: "rgbdns-frag-128d48d8a73e"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "ForwardZone"
kind: "struct"
start_line: 17
end_line: 26
---

# ForwardZone

- Fragment ID: `rgbdns-frag-128d48d8a73e`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 17-26
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-128d48d8a73e", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-128d48d8a73e: struct ForwardZone", "sourcePath": "src/dnscache_config.rs", "startLine": 17, "endLine": 26}
```

## Excerpt

<span id="rgbdns-frag-128d48d8a73e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-128d48d8a73e: struct ForwardZone

```rust
pub struct ForwardZone {
    pub name: String,
    pub servers: Vec<IpAddr>,
}

/// A root-hints path suitable for the recursive resolver.
///
/// Native djbdns `root/servers/@` files contain one address per line. Hickory
/// consumes a DNS master file, so legacy input is translated into a private
/// temporary master file and removed when this value is dropped.
```
