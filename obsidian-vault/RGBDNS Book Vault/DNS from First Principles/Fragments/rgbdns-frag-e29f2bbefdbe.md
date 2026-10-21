---
type: "code-fragment"
fragment_id: "rgbdns-frag-e29f2bbefdbe"
source_path: "tests/dnssec_tools.rs"
code_note: "DNS from First Principles/Code/tests/dnssec_tools.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "unix"
kind: "mod"
start_line: 2
end_line: 9
---

# unix

- Fragment ID: `rgbdns-frag-e29f2bbefdbe`
- Source file: [[DNS from First Principles/Code/tests/dnssec_tools.rs.source|tests/dnssec_tools.rs]]
- Lines: 2-9
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-e29f2bbefdbe", "codeNote": "DNS from First Principles/Code/tests/dnssec_tools.rs.source", "sourcePath": "tests/dnssec_tools.rs", "startLine": 2, "endLine": 9}
```

## Excerpt

<span id="rgbdns-frag-e29f2bbefdbe" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e29f2bbefdbe: mod unix

```rust
mod unix {
    use std::{
        fs,
        path::Path,
        process::{Command, Output},
        time::{SystemTime, UNIX_EPOCH},
    };

```
