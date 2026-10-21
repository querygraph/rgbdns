---
type: "code-fragment"
fragment_id: "rgbdns-frag-d8e5bf655ce7"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "apply_update"
kind: "fn"
start_line: 518
end_line: 573
---

# apply_update

- Fragment ID: `rgbdns-frag-d8e5bf655ce7`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 518-573
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d8e5bf655ce7", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 518, "endLine": 573}
```

## Excerpt

<span id="rgbdns-frag-d8e5bf655ce7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d8e5bf655ce7: fn apply_update

```rust
fn apply_update(
    message: &ParsedUpdate,
    policy: &Policy,
    overlay: &mut Overlay,
) -> std::result::Result<(), u16> {
    for prerequisite in &message.prerequisites {
        if !authorized(policy, &prerequisite.name) || prerequisite.typ != TYPE_TXT {
            return Err(5);
        }
        let exists = overlay
            .get(&prerequisite.name)
            .is_some_and(|values| !values.is_empty());
        match (
            prerequisite.class,
            prerequisite.ttl,
            prerequisite.rdata.is_empty(),
        ) {
            (CLASS_ANY, 0, true) if !exists => return Err(8),
            (CLASS_NONE, 0, true) if exists => return Err(7),
            (CLASS_ANY | CLASS_NONE, 0, true) => {}
            _ => return Err(4),
        }
    }
    let mut candidate = overlay.clone();
    for update in &message.updates {
        if !authorized(policy, &update.name) || update.typ != TYPE_TXT {
            return Err(5);
        }
        match update.class {
            CLASS_IN if update.ttl != 0 => {
                let value = decode_txt(&update.rdata).map_err(|_| 1u16)?;
                candidate
                    .entry(update.name.clone())
                    .or_default()
                    .insert(value);
            }
            CLASS_NONE if update.ttl == 0 => {
                let value = decode_txt(&update.rdata).map_err(|_| 1u16)?;
                if let Some(values) = candidate.get_mut(&update.name) {
                    values.remove(&value);
                    if values.is_empty() {
                        candidate.remove(&update.name);
                    }
                }
            }
            CLASS_ANY if update.ttl == 0 && update.rdata.is_empty() => {
                candidate.remove(&update.name);
            }
            _ => return Err(1),
        }
    }
    validate_overlay_counts(&candidate).map_err(|_| 5u16)?;
    *overlay = candidate;
    Ok(())
}

```
