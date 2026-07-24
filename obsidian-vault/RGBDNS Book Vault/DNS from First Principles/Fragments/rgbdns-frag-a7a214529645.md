---
type: "code-fragment"
fragment_id: "rgbdns-frag-a7a214529645"
source_path: "src/tinydns_edit.rs"
code_note: "DNS from First Principles/Code/src/tinydns_edit.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "add"
kind: "fn"
start_line: 43
end_line: 123
---

# add

- Fragment ID: `rgbdns-frag-a7a214529645`
- Source file: [[DNS from First Principles/Code/src/tinydns_edit.rs.source|src/tinydns_edit.rs]]
- Lines: 43-123
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-a7a214529645", "codeNote": "DNS from First Principles/Code/src/tinydns_edit.rs.source", "heading": "rgbdns-frag-a7a214529645: fn add", "sourcePath": "src/tinydns_edit.rs", "startLine": 43, "endLine": 123}
```

## Excerpt

<span id="rgbdns-frag-a7a214529645" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a7a214529645: fn add

```rust
pub fn add(
    data: &Path,
    temporary: &Path,
    mode: Mode,
    target: Name,
    address: Address,
) -> Result<()> {
    if matches!(mode, Mode::Host6 | Mode::Alias6) != matches!(address, Address::V6(_)) {
        return Err(Error::Format("address family does not match edit mode"));
    }
    if data == temporary {
        return Err(Error::Format("data and temporary paths must differ"));
    }
    let contents = fs::read_to_string(data)?;
    let mut used = [false; 26];
    let mut ttl = match mode {
        Mode::Ns | Mode::ChildNs => 259_200,
        _ => 86_400,
    };
    for raw in contents.lines() {
        let line = raw.trim_end_matches([' ', '\t', '\r']);
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let marker = line.as_bytes()[0];
        let fields = split_fields(&line[1..]);
        match mode {
            Mode::Ns | Mode::ChildNs => {
                let wanted = if mode == Mode::Ns { b'.' } else { b'&' };
                if marker == wanted && name_field(&fields, 0).as_ref() == Some(&target) {
                    ttl = number(&fields, 3, 259_200);
                    mark_slot(&mut used, &fields, 2, "ns", &target);
                }
            }
            Mode::Host if marker == b'=' => {
                if name_field(&fields, 0).as_ref() == Some(&target) {
                    return Err(Error::InvalidRecord("host name already used".into()));
                }
                if fields
                    .get(1)
                    .and_then(|value| value.parse::<Ipv4Addr>().ok())
                    == match address {
                        Address::V4(address) => Some(address),
                        Address::V6(_) => None,
                    }
                {
                    return Err(Error::InvalidRecord("IP address already used".into()));
                }
            }
            Mode::Mx if marker == b'@' && name_field(&fields, 0).as_ref() == Some(&target) => {
                ttl = number(&fields, 4, 86_400);
                mark_slot(&mut used, &fields, 2, "mx", &target);
            }
            Mode::Host6 if marker == b'6' => {
                if name_field(&fields, 0).as_ref() == Some(&target) {
                    return Err(Error::InvalidRecord("host name already used".into()));
                }
                if fields.get(1).and_then(|value| parse_flat_ipv6(value).ok())
                    == match address {
                        Address::V6(address) => Some(address),
                        Address::V4(_) => None,
                    }
                {
                    return Err(Error::InvalidRecord("IPv6 address already used".into()));
                }
            }
            _ => {}
        }
    }
    let owner = target.to_string().trim_end_matches('.').to_owned();
    let line = match mode {
        Mode::Ns | Mode::ChildNs | Mode::Mx => {
            let slot = used
                .iter()
                .position(|used| !used)
                .ok_or_else(|| Error::InvalidRecord("too many records for that domain".into()))?;
            let letter = char::from(b'a' + slot as u8);
            match mode {
                Mode::Ns => format!(".{owner}:{}:{letter}:{ttl}", display_address(address)),
                Mode::ChildNs => {
                    format!("&{owner}:{}:{letter}:{ttl}", display_address(address))
```
