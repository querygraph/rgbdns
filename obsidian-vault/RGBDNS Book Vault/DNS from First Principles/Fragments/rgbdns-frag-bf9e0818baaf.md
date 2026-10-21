---
type: "code-fragment"
fragment_id: "rgbdns-frag-bf9e0818baaf"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "truncate"
kind: "fn"
start_line: 519
end_line: 546
---

# truncate

- Fragment ID: `rgbdns-frag-bf9e0818baaf`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 519-546
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-bf9e0818baaf", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 519, "endLine": 546}
```

## Excerpt

<span id="rgbdns-frag-bf9e0818baaf" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bf9e0818baaf: fn truncate

```rust
fn truncate(mut response: Message, limit: usize) -> Result<Vec<u8>> {
    let full = response.encode()?;
    if full.len() <= limit {
        return Ok(full);
    }
    response.flags |= 0x0200;
    loop {
        let wire = response.encode()?;
        if wire.len() <= limit {
            return Ok(wire);
        }
        if remove_tail_rrset(&mut response.additionals, true)
            || remove_tail_rrset(&mut response.authorities, false)
            || remove_tail_rrset(&mut response.answers, false)
        {
            continue;
        }
        if response.additionals.pop().is_some() {
            continue;
        }
        if !response.questions.is_empty() {
            response.questions.clear();
            continue;
        }
        return Err(Error::Format("DNS response cannot fit transport limit"));
    }
}

```
