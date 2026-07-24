---
type: "code-fragment"
fragment_id: "rgbdns-frag-7d0ccc73f8dd"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "tests"
kind: "mod"
start_line: 353
end_line: 357
---

# tests

- Fragment ID: `rgbdns-frag-7d0ccc73f8dd`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 353-357
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7d0ccc73f8dd", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-7d0ccc73f8dd: mod tests", "sourcePath": "src/axfr.rs", "startLine": 353, "endLine": 357}
```

## Excerpt

<span id="rgbdns-frag-7d0ccc73f8dd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7d0ccc73f8dd: mod tests

```rust
mod tests {
    use super::*;
    use crate::zone::Lookup;
    use std::{net::Ipv4Addr, time::SystemTime};

```
