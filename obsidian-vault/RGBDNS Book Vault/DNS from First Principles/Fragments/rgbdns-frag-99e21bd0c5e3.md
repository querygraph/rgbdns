---
type: "code-fragment"
fragment_id: "rgbdns-frag-99e21bd0c5e3"
source_path: "src/client.rs"
code_note: "DNS from First Principles/Code/src/client.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "tests"
kind: "mod"
start_line: 148
end_line: 153
---

# tests

- Fragment ID: `rgbdns-frag-99e21bd0c5e3`
- Source file: [[DNS from First Principles/Code/src/client.rs.source|src/client.rs]]
- Lines: 148-153
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-99e21bd0c5e3", "codeNote": "DNS from First Principles/Code/src/client.rs.source", "heading": "rgbdns-frag-99e21bd0c5e3: mod tests", "sourcePath": "src/client.rs", "startLine": 148, "endLine": 153}
```

## Excerpt

<span id="rgbdns-frag-99e21bd0c5e3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-99e21bd0c5e3: mod tests

```rust
mod tests {
    use super::*;
    use crate::{RData, Record};
    use std::{net::Ipv4Addr, thread};

    #[test]
```
