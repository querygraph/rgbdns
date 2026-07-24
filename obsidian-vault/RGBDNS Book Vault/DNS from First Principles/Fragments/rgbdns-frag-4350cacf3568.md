---
type: "code-fragment"
fragment_id: "rgbdns-frag-4350cacf3568"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "tests"
kind: "mod"
start_line: 369
end_line: 378
---

# tests

- Fragment ID: `rgbdns-frag-4350cacf3568`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 369-378
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4350cacf3568", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-4350cacf3568: mod tests", "sourcePath": "src/server.rs", "startLine": 369, "endLine": 378}
```

## Excerpt

<span id="rgbdns-frag-4350cacf3568" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4350cacf3568: mod tests

```rust
mod tests {
    use super::*;
    use crate::{Name, Question, RData, Record, RecordType};
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream, UdpSocket},
        thread,
        time::Duration,
    };

```
