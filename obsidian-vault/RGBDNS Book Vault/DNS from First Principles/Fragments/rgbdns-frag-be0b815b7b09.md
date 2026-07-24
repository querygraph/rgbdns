---
type: "code-fragment"
fragment_id: "rgbdns-frag-be0b815b7b09"
source_path: "src/multilog.rs"
code_note: "DNS from First Principles/Code/src/multilog.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "parse"
kind: "fn"
start_line: 25
end_line: 61
---

# parse

- Fragment ID: `rgbdns-frag-be0b815b7b09`
- Source file: [[DNS from First Principles/Code/src/multilog.rs.source|src/multilog.rs]]
- Lines: 25-61
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-be0b815b7b09", "codeNote": "DNS from First Principles/Code/src/multilog.rs.source", "heading": "rgbdns-frag-be0b815b7b09: fn parse", "sourcePath": "src/multilog.rs", "startLine": 25, "endLine": 61}
```

## Excerpt

<span id="rgbdns-frag-be0b815b7b09" class="rgbdns-fragment-target"></span>
### rgbdns-frag-be0b815b7b09: fn parse

```rust
    pub fn parse(arguments: &[String]) -> Result<Self, String> {
        let mut timestamp = false;
        let mut max_size = DEFAULT_MAX_SIZE;
        let mut retain = DEFAULT_RETAIN;
        let mut directories = Vec::new();
        for argument in arguments {
            if argument == "t" {
                timestamp = true;
            } else if let Some(value) = argument.strip_prefix('s') {
                max_size = parse_bounded(value, 1, MAX_SIZE, "log size")?;
            } else if let Some(value) = argument.strip_prefix('n') {
                retain = parse_bounded(value, 1, MAX_RETAIN, "retention count")?;
            } else if argument.starts_with('-')
                || argument.starts_with('+')
                || argument.starts_with('e')
                || argument.starts_with('E')
            {
                return Err(format!("unsupported multilog selector: {argument}"));
            } else {
                directories.push(PathBuf::from(argument));
            }
        }
        if directories.is_empty() {
            return Err("multilog requires at least one log directory".into());
        }
        if directories.len() > MAX_DESTINATIONS {
            return Err("too many multilog destinations".into());
        }
        Ok(Self {
            timestamp,
            max_size,
            retain,
            directories,
        })
    }
}

```
