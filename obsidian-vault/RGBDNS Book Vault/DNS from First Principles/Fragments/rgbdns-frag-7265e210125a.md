---
type: "code-fragment"
fragment_id: "rgbdns-frag-7265e210125a"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "admin_update"
kind: "fn"
start_line: 110
end_line: 139
---

# admin_update

- Fragment ID: `rgbdns-frag-7265e210125a`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 110-139
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7265e210125a", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 110, "endLine": 139}
```

## Excerpt

<span id="rgbdns-frag-7265e210125a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7265e210125a: fn admin_update

```rust
pub fn admin_update(
    config: impl AsRef<Path>,
    address: SocketAddr,
    zone: &Name,
    owner: &Name,
    action: AdminAction<'_>,
) -> Result<()> {
    let policies = parse_config_without_zone(&fs::read_to_string(config)?)?;
    let policy = policies
        .values()
        .find(|policy| policy.zone == *zone && authorized(policy, owner))
        .ok_or(Error::Format("no ACME key authorizes this owner"))?;
    let wire = admin_wire(policy, zone, owner, action)?;
    let mut stream = TcpStream::connect_timeout(&address, std::time::Duration::from_secs(10))?;
    stream.set_read_timeout(Some(std::time::Duration::from_secs(10)))?;
    stream.write_all(&(wire.len() as u16).to_be_bytes())?;
    stream.write_all(&wire)?;
    let mut length = [0; 2];
    stream.read_exact(&mut length)?;
    let mut response = vec![0; usize::from(u16::from_be_bytes(length))];
    stream.read_exact(&mut response)?;
    if response.len() < 12 || response[3] & 0x0f != 0 {
        return Err(Error::InvalidRecord(format!(
            "ACME UPDATE failed with DNS rcode {}",
            response.get(3).copied().unwrap_or(0) & 0x0f
        )));
    }
    Ok(())
}

```
