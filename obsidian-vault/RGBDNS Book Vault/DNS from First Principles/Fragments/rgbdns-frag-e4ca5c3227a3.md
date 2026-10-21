---
type: "code-fragment"
fragment_id: "rgbdns-frag-e4ca5c3227a3"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "write_zone"
kind: "fn"
start_line: 364
end_line: 395
---

# write_zone

- Fragment ID: `rgbdns-frag-e4ca5c3227a3`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 364-395
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e4ca5c3227a3", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 364, "endLine": 395}
```

## Excerpt

<span id="rgbdns-frag-e4ca5c3227a3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e4ca5c3227a3: fn write_zone

```rust
pub fn write_zone(zone: &Zone, output: &Path, temporary: &Path) -> Result<()> {
    if output == temporary {
        return Err(Error::Format("zone output and temporary paths must differ"));
    }
    let (records, anames) = zone.materialization_input()?;
    let mut file = File::create(temporary)?;
    let write_result: Result<()> = (|| {
        for record in &records {
            writeln!(file, "{}", tinydns_line(record)?)?;
        }
        for (owner, aname) in anames {
            writeln!(
                file,
                "A{}:{}:{}",
                tinydns_name(&owner),
                tinydns_name(&aname.target),
                aname.ttl
            )?;
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
