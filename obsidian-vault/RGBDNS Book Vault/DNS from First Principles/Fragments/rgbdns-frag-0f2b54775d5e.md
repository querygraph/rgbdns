---
type: "code-fragment"
fragment_id: "rgbdns-frag-0f2b54775d5e"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "with_tail_records_removed"
kind: "fn"
start_line: 361
end_line: 386
---

# with_tail_records_removed

- Fragment ID: `rgbdns-frag-0f2b54775d5e`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 361-386
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-0f2b54775d5e", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "heading": "rgbdns-frag-0f2b54775d5e: fn with_tail_records_removed", "sourcePath": "src/server.rs", "startLine": 361, "endLine": 386}
```

## Excerpt

<span id="rgbdns-frag-0f2b54775d5e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0f2b54775d5e: fn with_tail_records_removed

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
