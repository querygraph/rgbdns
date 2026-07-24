---
type: "code-fragment"
fragment_id: "rgbdns-frag-cd7bdf864d6f"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "tai_to_unix"
kind: "fn"
start_line: 177
end_line: 189
---

# tai_to_unix

- Fragment ID: `rgbdns-frag-cd7bdf864d6f`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 177-189
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-cd7bdf864d6f", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-cd7bdf864d6f: fn tai_to_unix", "sourcePath": "src/tai64.rs", "startLine": 177, "endLine": 189}
```

## Excerpt

<span id="rgbdns-frag-cd7bdf864d6f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cd7bdf864d6f: fn tai_to_unix

```rust
fn tai_to_unix(tai_seconds: i64) -> i64 {
    let mut offset = INITIAL_TAI_UTC_OFFSET;
    for transition in LEAP_TRANSITIONS {
        let next_offset = offset + 1;
        if tai_seconds < transition + next_offset {
            break;
        }
        offset = next_offset;
    }
    tai_seconds - offset
}

#[cfg(test)]
```
