---
type: "code-fragment"
fragment_id: "rgbdns-frag-f83a15cfd75c"
source_path: "src/server.rs"
code_note: "DNS from First Principles/Code/src/server.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "remove_tail_rrset"
kind: "fn"
start_line: 547
end_line: 592
---

# remove_tail_rrset

- Fragment ID: `rgbdns-frag-f83a15cfd75c`
- Source file: [[DNS from First Principles/Code/src/server.rs.source|src/server.rs]]
- Lines: 547-592
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f83a15cfd75c", "codeNote": "DNS from First Principles/Code/src/server.rs.source", "sourcePath": "src/server.rs", "startLine": 547, "endLine": 592}
```

## Excerpt

<span id="rgbdns-frag-f83a15cfd75c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f83a15cfd75c: fn remove_tail_rrset

```rust
fn remove_tail_rrset(records: &mut Vec<crate::Record>, preserve_opt: bool) -> bool {
    let Some(index) = records
        .iter()
        .rposition(|record| !preserve_opt || record.rr_type() != crate::RecordType::Opt)
    else {
        return false;
    };
    let name = records[index].name.clone();
    let record_type = match &records[index].data {
        crate::RData::Opaque(crate::RecordType::Rrsig, bytes) if bytes.len() >= 2 => {
            crate::RecordType::from_code(u16::from_be_bytes([bytes[0], bytes[1]]))
        }
        _ => records[index].rr_type(),
    };
    let signed = records.iter().any(|record| {
        record.name == name
            && matches!(
                &record.data,
                crate::RData::Opaque(crate::RecordType::Rrsig, bytes)
                    if bytes.len() >= 2
                        && crate::RecordType::from_code(u16::from_be_bytes([bytes[0], bytes[1]]))
                            == record_type
            )
    });
    if !signed && records[index].rr_type() != crate::RecordType::Rrsig {
        records.remove(index);
        return true;
    }
    records.retain(|record| {
        if record.name != name {
            return true;
        }
        if record.rr_type() == record_type {
            return false;
        }
        !matches!(
            &record.data,
            crate::RData::Opaque(crate::RecordType::Rrsig, bytes)
                if bytes.len() >= 2
                    && crate::RecordType::from_code(u16::from_be_bytes([bytes[0], bytes[1]]))
                        == record_type
        )
    });
    true
}

```
