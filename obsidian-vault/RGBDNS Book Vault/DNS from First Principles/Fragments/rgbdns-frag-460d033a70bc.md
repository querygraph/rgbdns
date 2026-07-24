---
type: "code-fragment"
fragment_id: "rgbdns-frag-460d033a70bc"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "zone"
kind: "mod"
start_line: 21
end_line: 26
---

# zone

- Fragment ID: `rgbdns-frag-460d033a70bc`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 21-26
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-460d033a70bc", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "heading": "rgbdns-frag-460d033a70bc: mod zone", "sourcePath": "src/lib.rs", "startLine": 21, "endLine": 26}
```

## Excerpt

<span id="rgbdns-frag-460d033a70bc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-460d033a70bc: mod zone

```rust
pub mod zone;

pub use name::Name;
pub use packet::{Message, Question, RData, Record, RecordType};

#[derive(Debug)]
```
