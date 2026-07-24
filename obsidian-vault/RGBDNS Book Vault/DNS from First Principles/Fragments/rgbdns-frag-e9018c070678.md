---
type: "code-fragment"
fragment_id: "rgbdns-frag-e9018c070678"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "configure"
kind: "fn"
start_line: 20
end_line: 100
---

# configure

- Fragment ID: `rgbdns-frag-e9018c070678`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 20-100
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-e9018c070678", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-e9018c070678: fn configure", "sourcePath": "src/conf.rs", "startLine": 20, "endLine": 100}
```

## Excerpt

<span id="rgbdns-frag-e9018c070678" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e9018c070678: fn configure

```rust
pub fn configure(service: Service, arguments: &[String]) -> Result<()> {
    let (user, log_user, directory, ip, extra) = match service {
        Service::Tinydns | Service::Pickdns | Service::Walldns if arguments.len() == 4 => (
            arguments[0].as_str(),
            arguments[1].as_str(),
            Path::new(&arguments[2]),
            arguments[3].as_str(),
            None,
        ),
        Service::Dnscache if arguments.len() == 3 || arguments.len() == 4 => (
            arguments[0].as_str(),
            arguments[1].as_str(),
            Path::new(&arguments[2]),
            arguments.get(3).map_or("127.0.0.1", String::as_str),
            None,
        ),
        Service::Rbldns if arguments.len() == 5 => (
            arguments[0].as_str(),
            arguments[1].as_str(),
            Path::new(&arguments[2]),
            arguments[3].as_str(),
            Some(arguments[4].as_str()),
        ),
        Service::Axfrdns if arguments.len() == 5 => (
            arguments[0].as_str(),
            arguments[1].as_str(),
            Path::new(&arguments[2]),
            arguments[4].as_str(),
            Some(arguments[3].as_str()),
        ),
        _ => return Err(Error::Format("invalid service configuration arguments")),
    };
    if !directory.is_absolute()
        || extra.is_some_and(|path| {
            matches!(service, Service::Axfrdns) && !Path::new(path).is_absolute()
        })
    {
        return Err(Error::Format("service directories must be absolute"));
    }
    fs::create_dir(directory)?;
    make_log(directory, log_user)?;
    fs::create_dir(directory.join("env"))?;
    write_file(
        &directory.join("env/IP"),
        format!("{ip}\n").as_bytes(),
        0o644,
    )?;

    let root = if matches!(service, Service::Axfrdns) {
        PathBuf::from(extra.unwrap()).join("root")
    } else {
        directory.join("root")
    };
    write_file(
        &directory.join("env/ROOT"),
        format!("{}\n", root.display()).as_bytes(),
        0o644,
    )?;
    if !matches!(service, Service::Axfrdns) {
        fs::create_dir(&root)?;
    }

    match service {
        Service::Tinydns => configure_tinydns(directory, &root)?,
        Service::Dnscache => configure_dnscache(directory, &root)?,
        Service::Rbldns => {
            write_file(
                &directory.join("env/BASE"),
                format!("{}\n", extra.unwrap()).as_bytes(),
                0o644,
            )?;
            write_file(&root.join("data"), b"", 0o644)?;
            write_file(
                &root.join("Makefile"),
                format!(
                    "data.cdb: data\n\t{}\n",
                    executable("rbldns-data")?.display()
                )
                .as_bytes(),
                0o644,
            )?;
```
