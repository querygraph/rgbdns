---
type: "code-fragment"
fragment_id: "rgbdns-frag-b2afa3491e16"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "tests"
kind: "mod"
start_line: 506
end_line: 510
---

# tests

- Fragment ID: `rgbdns-frag-b2afa3491e16`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 506-510
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b2afa3491e16", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 506, "endLine": 510}
```

## Excerpt

<span id="rgbdns-frag-b2afa3491e16" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b2afa3491e16: mod tests

```rust
mod tests {
    use super::*;
    use crate::{dnssec, zone::Lookup};
    use std::{net::Ipv4Addr, time::SystemTime};

```
