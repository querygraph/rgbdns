---
type: "code-fragment"
fragment_id: "rgbdns-frag-d375ebc0f305"
source_path: "src/name.rs"
code_note: "DNS from First Principles/Code/src/name.rs.source"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
symbol: "hash"
kind: "fn"
start_line: 33
end_line: 43
---

# hash

- Fragment ID: `rgbdns-frag-d375ebc0f305`
- Source file: [[DNS from First Principles/Code/src/name.rs.source|src/name.rs]]
- Lines: 33-43
- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d375ebc0f305", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-d375ebc0f305: fn hash", "sourcePath": "src/name.rs", "startLine": 33, "endLine": 43}
```

## Excerpt

<span id="rgbdns-frag-d375ebc0f305" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d375ebc0f305: fn hash

```rust
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.len().hash(state);
        for label in &self.0 {
            label.len().hash(state);
            for byte in label {
                byte.to_ascii_lowercase().hash(state);
            }
        }
    }
}

```
