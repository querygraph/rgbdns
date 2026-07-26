---
type: "code-fragment"
fragment_id: "rgbdns-frag-2ff6549bf9ff"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "write_tinydns"
kind: "fn"
start_line: 280
end_line: 304
---

# write_tinydns

- Fragment ID: `rgbdns-frag-2ff6549bf9ff`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 280-304
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2ff6549bf9ff", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-2ff6549bf9ff: fn write_tinydns", "sourcePath": "src/axfr.rs", "startLine": 280, "endLine": 304}
```

## Excerpt

<span id="rgbdns-frag-2ff6549bf9ff" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2ff6549bf9ff: fn write_tinydns

```rust
pub fn write_tinydns(records: &[Record], output: &Path, temporary: &Path) -> Result<()> {
    if output == temporary {
        return Err(Error::Format("AXFR output and temporary paths must differ"));
    }
    let mut file = File::create(temporary)?;
    let write_result: Result<()> = (|| {
        for (index, record) in records.iter().enumerate() {
            if index + 1 == records.len() && record.rr_type() == RecordType::Soa {
                continue;
            }
            writeln!(file, "{}", tinydns_line(record)?)?;
        }
        file.sync_all()?;
        Ok(())
    })();
    if let Err(error) = write_result {
        drop(file);
        let _ = fs::remove_file(temporary);
        return Err(error);
    }
    drop(file);
    fs::rename(temporary, output)?;
    Ok(())
}

```
