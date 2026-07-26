---
type: "code-file"
source_path: "tests/cdb_golden.rs"
language: "rust"
subsystem: "Tests and performance"
line_count: 89
fragment_count: 7
rgbdns_commit: "79502939"
---

# tests/cdb_golden.rs

- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]
- Source path: `tests/cdb_golden.rs`
- Lines: 89
- Summary: Source file in the Tests and performance subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-ae1791d3fe93|path]]: lines 8-18
- [[DNS from First Principles/Fragments/rgbdns-frag-2c363bf23160|entries]]: lines 19-44
- [[DNS from First Principles/Fragments/rgbdns-frag-526964b7290d|hex]]: lines 45-48
- [[DNS from First Principles/Fragments/rgbdns-frag-8ab25cad4553|expected]]: lines 49-55
- [[DNS from First Principles/Fragments/rgbdns-frag-b5367143d717|tinydns_entries_match_patched_c_golden]]: lines 56-67
- [[DNS from First Principles/Fragments/rgbdns-frag-8703d812c744|rbldns_entries_match_original_c_golden]]: lines 68-79
- [[DNS from First Principles/Fragments/rgbdns-frag-02993ea955ac|pickdns_entries_match_original_c_golden]]: lines 80-89

## Full Source

```rust
use rgbdns::{cdb, pick, rbl, zone::Zone};
use std::{
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

fn path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "rgbdns-golden-{label}-{}-{}.cdb",
        std::process::id(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn entries(path: &Path) -> Vec<String> {
    let bytes = fs::read(path).unwrap();
    let data_end = (0..256)
        .map(|index| {
            u32::from_le_bytes(bytes[index * 8..index * 8 + 4].try_into().unwrap()) as usize
        })
        .min()
        .unwrap();
    let mut position = 2048;
    let mut entries = Vec::new();
    while position < data_end {
        let key_length =
            u32::from_le_bytes(bytes[position..position + 4].try_into().unwrap()) as usize;
        let value_length =
            u32::from_le_bytes(bytes[position + 4..position + 8].try_into().unwrap()) as usize;
        position += 8;
        let key = &bytes[position..position + key_length];
        position += key_length;
        let value = &bytes[position..position + value_length];
        position += value_length;
        entries.push(format!("{}:{}", hex(key), hex(value)));
    }
    entries.sort();
    entries
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn expected(contents: &str) -> Vec<String> {
    let mut lines = contents.lines().map(str::to_owned).collect::<Vec<_>>();
    lines.sort();
    lines
}

#[test]
fn tinydns_entries_match_patched_c_golden() {
    let database = Zone::parse(include_str!("fixtures/tinydns-data")).unwrap();
    let path = path("tinydns");
    cdb::compile(&database, &path).unwrap();
    assert_eq!(
        entries(&path),
        expected(include_str!("fixtures/tinydns-cdb.entries"))
    );
    fs::remove_file(path).unwrap();
}

#[test]
fn rbldns_entries_match_original_c_golden() {
    let database = rbl::Database::parse(include_str!("fixtures/rbldns-data")).unwrap();
    let path = path("rbldns");
    rbl::compile(&database, &path).unwrap();
    assert_eq!(
        entries(&path),
        expected(include_str!("fixtures/rbldns-cdb.entries"))
    );
    fs::remove_file(path).unwrap();
}

#[test]
fn pickdns_entries_match_original_c_golden() {
    let database = pick::Database::parse(include_str!("fixtures/pickdns-data")).unwrap();
    let path = path("pickdns");
    pick::compile(&database, &path).unwrap();
    assert_eq!(
        entries(&path),
        expected(include_str!("fixtures/pickdns-cdb.entries"))
    );
    fs::remove_file(path).unwrap();
}
```
