---
type: "code-fragment"
fragment_id: "rgbdns-frag-3591e119df49"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "zone"
kind: "mod"
start_line: 22
end_line: 27
---

# zone

- Fragment ID: `rgbdns-frag-3591e119df49`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 22-27
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-3591e119df49", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-3591e119df49: mod zone", "sourcePath": "src/lib.rs", "startLine": 22, "endLine": 27}
```

## Excerpt

<span id="rgbdns-frag-3591e119df49" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3591e119df49: mod zone

```rust
pub mod zone;

pub use name::Name;
pub use packet::{Message, Question, RData, Record, RecordType};

#[derive(Debug)]
```
