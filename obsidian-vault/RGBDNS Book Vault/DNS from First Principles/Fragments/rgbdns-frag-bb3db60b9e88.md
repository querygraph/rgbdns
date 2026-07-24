---
type: "code-fragment"
fragment_id: "rgbdns-frag-bb3db60b9e88"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "with_tail_records_removed"
kind: "fn"
start_line: 320
end_line: 345
---

# with_tail_records_removed

- Fragment ID: `rgbdns-frag-bb3db60b9e88`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 320-345
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-bb3db60b9e88", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-bb3db60b9e88: fn with_tail_records_removed", "sourcePath": "src/server.rs", "startLine": 320, "endLine": 345}
```

## Excerpt

<span id="rgbdns-frag-bb3db60b9e88" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bb3db60b9e88: fn with_tail_records_removed

```rust
fn with_tail_records_removed(response: &Message, mut count: usize) -> Message {
    let mut candidate = response.clone();
    while count != 0
        && let Some(index) = candidate
            .additionals
            .iter()
            .rposition(|record| record.rr_type() != crate::RecordType::Opt)
    {
        candidate.additionals.remove(index);
        count -= 1;
    }
    while count != 0 && candidate.authorities.pop().is_some() {
        count -= 1;
    }
    while count != 0 && candidate.answers.pop().is_some() {
        count -= 1;
    }
    while count != 0 && candidate.additionals.pop().is_some() {
        count -= 1;
    }
    if count != 0 {
        candidate.questions.clear();
    }
    candidate
}

```
