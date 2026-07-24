---
type: "code-fragment"
fragment_id: "rgbdns-frag-578de589f37c"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "prepare"
kind: "fn"
start_line: 46
end_line: 98
---

# prepare

- Fragment ID: `rgbdns-frag-578de589f37c`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 46-98
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-578de589f37c", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-578de589f37c: fn prepare", "sourcePath": "src/dnscache_config.rs", "startLine": 46, "endLine": 98}
```

## Excerpt

<span id="rgbdns-frag-578de589f37c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-578de589f37c: fn prepare

```rust
    pub fn prepare(path: PathBuf) -> Result<Self> {
        let metadata = fs::metadata(&path)?;
        if metadata.len() > MAX_ROOTS_FILE {
            return Err(Error::Format("root hints file is too large"));
        }
        let contents = fs::read_to_string(&path)?;
        let lines = contents
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect::<Vec<_>>();
        if lines.is_empty() {
            return Err(Error::Format("root hints file contains no servers"));
        }

        let addresses = lines
            .iter()
            .map(|line| line.parse::<IpAddr>())
            .collect::<std::result::Result<Vec<_>, _>>();
        let Ok(addresses) = addresses else {
            // Reject mixed legacy/master input, but leave a DNS master file to
            // Hickory's strict parser for full syntax validation.
            if lines.iter().any(|line| line.parse::<IpAddr>().is_ok()) {
                return Err(Error::Format("mixed root hints file formats"));
            }
            return Ok(Self {
                path,
                temporary: false,
            });
        };
        if addresses.len() > MAX_ROOT_ADDRESSES {
            return Err(Error::Format("too many root server addresses"));
        }

        let mut master = String::new();
        for (index, address) in addresses.iter().enumerate() {
            let host = format!("root-{index}.rgbdns.invalid.");
            master.push_str(&format!(". 3600000 NS {host}\n"));
            let rr_type = if address.is_ipv4() { "A" } else { "AAAA" };
            master.push_str(&format!("{host} 3600000 {rr_type} {address}\n"));
        }
        let path = private_temporary_path()?;
        let result = write_private(&path, master.as_bytes());
        if result.is_err() {
            let _ = fs::remove_file(&path);
        }
        result?;
        Ok(Self {
            path,
            temporary: true,
        })
    }

```
