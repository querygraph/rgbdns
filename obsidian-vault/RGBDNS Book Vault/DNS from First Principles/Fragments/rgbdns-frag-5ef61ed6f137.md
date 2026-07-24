---
type: "code-fragment"
fragment_id: "rgbdns-frag-5ef61ed6f137"
source_path: "src/rbl.rs"
code_note: "DNS from First Principles/Code/src/rbl.rs.source"
language: "rust"
subsystem: "Specialized responders"
crate: "rgbdns"
symbol: "numeric_prefix"
kind: "fn"
start_line: 196
end_line: 220
---

# numeric_prefix

- Fragment ID: `rgbdns-frag-5ef61ed6f137`
- Source file: [[DNS from First Principles/Code/src/rbl.rs.source|src/rbl.rs]]
- Lines: 196-220
- Subsystem: [[DNS from First Principles/Subsystems/Specialized responders|Specialized responders]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-5ef61ed6f137", "codeNote": "DNS from First Principles/Code/src/rbl.rs.source", "heading": "rgbdns-frag-5ef61ed6f137: fn numeric_prefix", "sourcePath": "src/rbl.rs", "startLine": 196, "endLine": 220}
```

## Excerpt

<span id="rgbdns-frag-5ef61ed6f137" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5ef61ed6f137: fn numeric_prefix

```rust
pub(crate) fn numeric_prefix(name: &Name, suffix: &Name, maximum: usize) -> Option<Vec<u8>> {
    let labels = name.labels().collect::<Vec<_>>();
    let suffix_labels = suffix.labels().collect::<Vec<_>>();
    if labels.len() < suffix_labels.len()
        || labels[labels.len() - suffix_labels.len()..] != suffix_labels
    {
        return None;
    }
    let prefix = &labels[..labels.len() - suffix_labels.len()];
    if prefix.len() > maximum {
        return None;
    }
    prefix
        .iter()
        .map(|label| {
            let text = std::str::from_utf8(label).ok()?;
            if text.len() > 1 && text.starts_with('0') {
                return None;
            }
            text.parse::<u8>().ok()
        })
        .collect()
}

#[cfg(test)]
```
