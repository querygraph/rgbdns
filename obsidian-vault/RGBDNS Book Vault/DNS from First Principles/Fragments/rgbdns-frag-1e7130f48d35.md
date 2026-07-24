---
type: "code-fragment"
fragment_id: "rgbdns-frag-1e7130f48d35"
source_path: "src/dnscache_config.rs"
code_note: "DNS from First Principles/Code/src/dnscache_config.rs.source"
language: "rust"
subsystem: "Resolution and recursion"
crate: "rgbdns"
symbol: "load_forward_zones"
kind: "fn"
start_line: 112
end_line: 158
---

# load_forward_zones

- Fragment ID: `rgbdns-frag-1e7130f48d35`
- Source file: [[DNS from First Principles/Code/src/dnscache_config.rs.source|src/dnscache_config.rs]]
- Lines: 112-158
- Subsystem: [[DNS from First Principles/Subsystems/Resolution and recursion|Resolution and recursion]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-1e7130f48d35", "codeNote": "DNS from First Principles/Code/src/dnscache_config.rs.source", "heading": "rgbdns-frag-1e7130f48d35: fn load_forward_zones", "sourcePath": "src/dnscache_config.rs", "startLine": 112, "endLine": 158}
```

## Excerpt

<span id="rgbdns-frag-1e7130f48d35" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1e7130f48d35: fn load_forward_zones

```rust
pub fn load_forward_zones(directory: &Path) -> Result<Vec<ForwardZone>> {
    let mut zones = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        if zones.len() >= MAX_FORWARD_ZONES {
            return Err(Error::Format("too many forwarding zones"));
        }
        let file_type = entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| Error::Format("forwarding zone name is not UTF-8"))?;
        if name == "@" {
            continue;
        }
        if name.is_empty() || name.starts_with('.') || name.ends_with('.') {
            return Err(Error::Format("invalid forwarding zone filename"));
        }
        let metadata = entry.metadata()?;
        if metadata.len() > MAX_ROOTS_FILE {
            return Err(Error::Format("forwarding server file is too large"));
        }
        let contents = fs::read_to_string(entry.path())?;
        let servers = contents
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| {
                line.parse::<IpAddr>()
                    .map_err(|_| Error::Format("invalid forwarding server address"))
            })
            .collect::<Result<Vec<_>>>()?;
        if servers.is_empty() {
            return Err(Error::Format("forwarding zone contains no servers"));
        }
        if servers.len() > MAX_ROOT_ADDRESSES {
            return Err(Error::Format("too many forwarding server addresses"));
        }
        zones.push(ForwardZone { name, servers });
    }
    zones.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(zones)
}

```
