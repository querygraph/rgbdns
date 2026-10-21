---
type: "code-fragment"
fragment_id: "rgbdns-frag-a42b86cbda5b"
source_path: "src/lib.rs"
code_note: "DNS from First Principles/Code/src/lib.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "zone"
kind: "mod"
start_line: 24
end_line: 29
---

# zone

- Fragment ID: `rgbdns-frag-a42b86cbda5b`
- Source file: [[DNS from First Principles/Code/src/lib.rs.source|src/lib.rs]]
- Lines: 24-29
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a42b86cbda5b", "codeNote": "DNS from First Principles/Code/src/lib.rs.source", "sourcePath": "src/lib.rs", "startLine": 24, "endLine": 29}
```

## Excerpt

<span id="rgbdns-frag-a42b86cbda5b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a42b86cbda5b: mod zone

```rust
pub mod zone;

pub use name::Name;
pub use packet::{DnssecRData, Message, Question, RData, Record, RecordType};

#[derive(Debug)]
```
