---
type: "code-fragment"
fragment_id: "rgbdns-frag-59e1746cbb79"
source_path: "src/setuidgid.rs"
code_note: "DNS from First Principles/Code/src/setuidgid.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "resolve"
kind: "fn"
start_line: 15
end_line: 37
---

# resolve

- Fragment ID: `rgbdns-frag-59e1746cbb79`
- Source file: [[DNS from First Principles/Code/src/setuidgid.rs.source|src/setuidgid.rs]]
- Lines: 15-37
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-59e1746cbb79", "codeNote": "DNS from First Principles/Code/src/setuidgid.rs.source", "heading": "rgbdns-frag-59e1746cbb79: fn resolve", "sourcePath": "src/setuidgid.rs", "startLine": 15, "endLine": 37}
```

## Excerpt

<span id="rgbdns-frag-59e1746cbb79" class="rgbdns-fragment-target"></span>
### rgbdns-frag-59e1746cbb79: fn resolve

```rust
pub fn resolve(account: &str) -> Result<Identity, String> {
    if account.is_empty() || account.as_bytes().contains(&0) {
        return Err("invalid account name".into());
    }
    let (user_name, group_name) = account.split_once(':').unwrap_or((account, ""));
    let user = User::from_name(user_name)
        .map_err(|error| format!("account lookup failed for {user_name}: {error}"))?
        .ok_or_else(|| format!("unknown account: {user_name}"))?;
    let gid = if group_name.is_empty() {
        user.gid
    } else {
        Group::from_name(group_name)
            .map_err(|error| format!("group lookup failed for {group_name}: {error}"))?
            .ok_or_else(|| format!("unknown group: {group_name}"))?
            .gid
    };
    Ok(Identity {
        name: user.name,
        uid: user.uid,
        gid,
    })
}

```
