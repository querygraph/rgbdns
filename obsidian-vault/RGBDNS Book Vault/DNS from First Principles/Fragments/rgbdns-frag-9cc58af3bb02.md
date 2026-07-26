---
type: "code-fragment"
fragment_id: "rgbdns-frag-9cc58af3bb02"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "tests"
kind: "mod"
start_line: 302
end_line: 307
---

# tests

- Fragment ID: `rgbdns-frag-9cc58af3bb02`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 302-307
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-9cc58af3bb02", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-9cc58af3bb02: mod tests", "sourcePath": "src/cdb.rs", "startLine": 302, "endLine": 307}
```

## Excerpt

<span id="rgbdns-frag-9cc58af3bb02" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9cc58af3bb02: mod tests

```rust
mod tests {
    use super::*;
    use crate::zone::Lookup;
    use std::{net::Ipv4Addr, time::SystemTime};

    #[test]
```
