---
type: "code-fragment"
fragment_id: "rgbdns-frag-7fc73163aada"
source_path: "src/setuidgid.rs"
code_note: "DNS from First Principles/Code/src/setuidgid.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "drop_privileges"
kind: "fn"
start_line: 38
end_line: 64
---

# drop_privileges

- Fragment ID: `rgbdns-frag-7fc73163aada`
- Source file: [[DNS from First Principles/Code/src/setuidgid.rs.source|src/setuidgid.rs]]
- Lines: 38-64
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-7fc73163aada", "codeNote": "DNS from First Principles/Code/src/setuidgid.rs.source", "heading": "rgbdns-frag-7fc73163aada: fn drop_privileges", "sourcePath": "src/setuidgid.rs", "startLine": 38, "endLine": 64}
```

## Excerpt

<span id="rgbdns-frag-7fc73163aada" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7fc73163aada: fn drop_privileges

```rust
pub fn drop_privileges(identity: &Identity) -> Result<(), String> {
    if Uid::effective() == identity.uid
        && Uid::current() == identity.uid
        && Gid::effective() == identity.gid
        && Gid::current() == identity.gid
    {
        return Ok(());
    }
    #[cfg(target_vendor = "apple")]
    return Err("privilege dropping is unavailable on Apple platforms".into());
    #[cfg(not(target_vendor = "apple"))]
    {
        let name = CString::new(identity.name.as_bytes()).map_err(|_| "invalid account name")?;
        initgroups(&name, identity.gid).map_err(|error| error.to_string())?;
        setgid(identity.gid).map_err(|error| error.to_string())?;
        setuid(identity.uid).map_err(|error| error.to_string())?;
        if Uid::effective() != identity.uid
            || Uid::current() != identity.uid
            || Gid::effective() != identity.gid
            || Gid::current() != identity.gid
        {
            return Err("privilege drop verification failed".into());
        }
        Ok(())
    }
}

```
