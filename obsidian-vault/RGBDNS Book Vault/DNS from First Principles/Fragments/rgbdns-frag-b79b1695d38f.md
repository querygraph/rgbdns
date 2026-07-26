---
type: "code-fragment"
fragment_id: "rgbdns-frag-b79b1695d38f"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "truncate"
kind: "fn"
start_line: 319
end_line: 360
---

# truncate

- Fragment ID: `rgbdns-frag-b79b1695d38f`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 319-360
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-b79b1695d38f", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-b79b1695d38f: fn truncate", "sourcePath": "src/server.rs", "startLine": 319, "endLine": 360}
```

## Excerpt

<span id="rgbdns-frag-b79b1695d38f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b79b1695d38f: fn truncate

```rust
fn truncate(mut response: Message, limit: usize) -> Result<Vec<u8>> {
    let full = response.encode()?;
    if full.len() <= limit {
        return Ok(full);
    }
    response.flags |= 0x0200;
    let removable = response
        .additionals
        .iter()
        .filter(|record| record.rr_type() != crate::RecordType::Opt)
        .count()
        + response.authorities.len()
        + response.answers.len()
        + response
            .additionals
            .iter()
            .filter(|record| record.rr_type() == crate::RecordType::Opt)
            .count()
        + usize::from(!response.questions.is_empty());

    let mut low = 0;
    let mut high = removable;
    while low < high {
        let middle = low + (high - low) / 2;
        let candidate = with_tail_records_removed(&response, middle);
        if candidate.encode()?.len() <= limit {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    while low <= removable {
        let candidate = with_tail_records_removed(&response, low);
        let wire = candidate.encode()?;
        if wire.len() <= limit {
            return Ok(wire);
        }
        low += 1;
    }
    Err(Error::Format("DNS response cannot fit transport limit"))
}

```
