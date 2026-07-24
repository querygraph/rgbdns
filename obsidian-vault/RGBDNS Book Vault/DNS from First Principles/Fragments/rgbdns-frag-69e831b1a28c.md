---
type: "code-fragment"
fragment_id: "rgbdns-frag-69e831b1a28c"
source_path: "tests/rfc_conformance.rs"
code_note: "DNS from First Principles/Code/tests/rfc_conformance.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "support"
kind: "mod"
start_line: 1
end_line: 6
---

# support

- Fragment ID: `rgbdns-frag-69e831b1a28c`
- Source file: [[DNS from First Principles/Code/tests/rfc_conformance.rs.source|tests/rfc_conformance.rs]]
- Lines: 1-6
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-69e831b1a28c", "codeNote": "DNS from First Principles/Code/tests/rfc_conformance.rs.source", "heading": "rgbdns-frag-69e831b1a28c: mod support", "sourcePath": "tests/rfc_conformance.rs", "startLine": 1, "endLine": 6}
```

## Excerpt

<span id="rgbdns-frag-69e831b1a28c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-69e831b1a28c: mod support

```rust
mod support;

use rgbdns::{Message, RData, RecordType};
use support::{ID, extended_rcode, opt, query, rcode, response, zone};

#[test]
```
