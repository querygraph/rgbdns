---
type: "code-fragment"
fragment_id: "rgbdns-frag-c270157bbc26"
source_path: "src/special.rs"
code_note: "DNS from First Principles/Code/src/special.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "serve"
kind: "fn"
start_line: 8
end_line: 10
---

# serve

- Fragment ID: `rgbdns-frag-c270157bbc26`
- Source file: [[DNS from First Principles/Code/src/special.rs.source|src/special.rs]]
- Lines: 8-10
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c270157bbc26", "codeNote": "DNS from First Principles/Code/src/special.rs.source", "heading": "rgbdns-frag-c270157bbc26: fn serve", "sourcePath": "src/special.rs", "startLine": 8, "endLine": 10}
```

## Excerpt

<span id="rgbdns-frag-c270157bbc26" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c270157bbc26: fn serve

```rust
pub fn serve(address: &str, handler: Arc<Handler>) -> Result<()> {
    crate::transport::serve(address, handler)
}
```
