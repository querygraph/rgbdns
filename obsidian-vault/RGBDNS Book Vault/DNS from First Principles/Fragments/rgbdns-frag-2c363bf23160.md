---
type: "code-fragment"
fragment_id: "rgbdns-frag-2c363bf23160"
source_path: "tests/cdb_golden.rs"
code_note: "DNS from First Principles/Code/tests/cdb_golden.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "entries"
kind: "fn"
start_line: 19
end_line: 44
---

# entries

- Fragment ID: `rgbdns-frag-2c363bf23160`
- Source file: [[DNS from First Principles/Code/tests/cdb_golden.rs.source|tests/cdb_golden.rs]]
- Lines: 19-44
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-2c363bf23160", "codeNote": "DNS from First Principles/Code/tests/cdb_golden.rs.source", "heading": "rgbdns-frag-2c363bf23160: fn entries", "sourcePath": "tests/cdb_golden.rs", "startLine": 19, "endLine": 44}
```

## Excerpt

<span id="rgbdns-frag-2c363bf23160" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2c363bf23160: fn entries

```rust
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

```
