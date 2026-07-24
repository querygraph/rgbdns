---
type: "code-file"
source_path: "src/setuidgid.rs"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
line_count: 107
fragment_count: 8
rgbdns_commit: "472c2087"
---

# src/setuidgid.rs

- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]
- Source path: `src/setuidgid.rs`
- Lines: 107
- Summary: Safe identity lookup and privilege dropping for `setuidgid`.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-7922cf070768|Identity]]: lines 9-14
- [[DNS from First Principles/Fragments/rgbdns-frag-59e1746cbb79|resolve]]: lines 15-37
- [[DNS from First Principles/Fragments/rgbdns-frag-7fc73163aada|drop_privileges]]: lines 38-64
- [[DNS from First Principles/Fragments/rgbdns-frag-7b46347386bc|command]]: lines 65-77
- [[DNS from First Principles/Fragments/rgbdns-frag-d7301c7d3da0|exec]]: lines 78-86
- [[DNS from First Principles/Fragments/rgbdns-frag-6beec8a9bd79|tests]]: lines 87-90
- [[DNS from First Principles/Fragments/rgbdns-frag-7187ba54fb70|resolves_the_current_account_and_optional_group]]: lines 91-99
- [[DNS from First Principles/Fragments/rgbdns-frag-d642db76538b|validates_exec_arguments]]: lines 100-107

## Full Source

```rust
//! Safe identity lookup and privilege dropping for `setuidgid`.

use nix::unistd::{Gid, Group, Uid, User};
#[cfg(not(target_vendor = "apple"))]
use nix::unistd::{initgroups, setgid, setuid};
use std::ffi::{CStr, CString};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Identity {
    pub name: String,
    pub uid: Uid,
    pub gid: Gid,
}

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

pub fn command(arguments: &[String]) -> Result<(CString, Vec<CString>), String> {
    if arguments.is_empty() {
        return Err("setuidgid requires a program".into());
    }
    let arguments = arguments
        .iter()
        .map(|argument| {
            CString::new(argument.as_bytes()).map_err(|_| "program argument contains NUL".into())
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((arguments[0].clone(), arguments))
}

pub fn exec(program: &CStr, arguments: &[CString]) -> Result<(), String> {
    let references = arguments.iter().map(CString::as_c_str).collect::<Vec<_>>();
    match nix::unistd::execvp(program, &references) {
        Ok(never) => match never {},
        Err(error) => Err(error.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_the_current_account_and_optional_group() {
        let current = User::from_uid(Uid::effective()).unwrap().unwrap();
        let identity = resolve(&current.name).unwrap();
        assert_eq!(identity.uid, Uid::effective());
        assert_eq!(identity.gid, current.gid);
        assert!(resolve("rgbdns-user-that-cannot-exist").is_err());
    }

    #[test]
    fn validates_exec_arguments() {
        let (program, arguments) = command(&["printf".into(), "ok".into()]).unwrap();
        assert_eq!(program.as_bytes(), b"printf");
        assert_eq!(arguments.len(), 2);
        assert!(command(&[]).is_err());
        assert!(command(&["bad\0program".into()]).is_err());
    }
}
```
