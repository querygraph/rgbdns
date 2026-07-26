---
type: "code-fragment"
fragment_id: "rgbdns-frag-7486a2a93364"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "tests"
kind: "mod"
start_line: 370
end_line: 374
---

# tests

- Fragment ID: `rgbdns-frag-7486a2a93364`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 370-374
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7486a2a93364", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-7486a2a93364: mod tests", "sourcePath": "src/axfr.rs", "startLine": 370, "endLine": 374}
```

## Excerpt

<span id="rgbdns-frag-7486a2a93364" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7486a2a93364: mod tests

```rust
mod tests {
    use super::*;
    use crate::zone::Lookup;
    use std::{net::Ipv4Addr, time::SystemTime};

```
