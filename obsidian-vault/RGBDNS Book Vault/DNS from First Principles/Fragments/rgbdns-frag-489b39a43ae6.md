---
type: "code-fragment"
fragment_id: "rgbdns-frag-489b39a43ae6"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "tests"
kind: "mod"
start_line: 270
end_line: 275
---

# tests

- Fragment ID: `rgbdns-frag-489b39a43ae6`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 270-275
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-489b39a43ae6", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-489b39a43ae6: mod tests", "sourcePath": "src/cdb.rs", "startLine": 270, "endLine": 275}
```

## Excerpt

<span id="rgbdns-frag-489b39a43ae6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-489b39a43ae6: mod tests

```rust
mod tests {
    use super::*;
    use crate::zone::Lookup;
    use std::{net::Ipv4Addr, time::SystemTime};

    #[test]
```
