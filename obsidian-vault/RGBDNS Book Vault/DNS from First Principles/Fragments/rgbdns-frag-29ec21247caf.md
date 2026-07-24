---
type: "code-fragment"
fragment_id: "rgbdns-frag-29ec21247caf"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "mark_slot"
kind: "fn"
start_line: 177
end_line: 197
---

# mark_slot

- Fragment ID: `rgbdns-frag-29ec21247caf`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 177-197
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-29ec21247caf", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-29ec21247caf: fn mark_slot", "sourcePath": "src/tinydns_edit.rs", "startLine": 177, "endLine": 197}
```

## Excerpt

<span id="rgbdns-frag-29ec21247caf" class="rgbdns-fragment-target"></span>
### rgbdns-frag-29ec21247caf: fn mark_slot

```rust
fn mark_slot(used: &mut [bool; 26], fields: &[String], index: usize, role: &str, owner: &Name) {
    let Some(value) = fields.get(index) else {
        return;
    };
    let expanded = if value.contains('.') {
        value.clone()
    } else {
        format!("{value}.{role}.{owner}")
    };
    let Ok(expanded) = expanded.parse::<Name>() else {
        return;
    };
    for (index, slot) in used.iter_mut().enumerate() {
        let candidate = format!("{}.{role}.{owner}", char::from(b'a' + index as u8));
        if candidate.parse::<Name>().ok().as_ref() == Some(&expanded) {
            *slot = true;
            break;
        }
    }
}

```
