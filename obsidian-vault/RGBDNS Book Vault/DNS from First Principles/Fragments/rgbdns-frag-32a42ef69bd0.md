---
type: "code-fragment"
fragment_id: "rgbdns-frag-32a42ef69bd0"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "subsystem_for"
kind: "def"
start_line: 205
end_line: 234
---

# subsystem_for

- Fragment ID: `rgbdns-frag-32a42ef69bd0`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 205-234
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-32a42ef69bd0", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 205, "endLine": 234}
```

## Excerpt

<span id="rgbdns-frag-32a42ef69bd0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-32a42ef69bd0: def subsystem_for

```python
def subsystem_for(path: str) -> tuple[str, str | None]:
    parts = PurePosixPath(path).parts
    name = PurePosixPath(path).name
    if parts and parts[0] == "src":
        if len(parts) >= 2 and parts[1] == "bin":
            return "Command-line programs", name.removesuffix(".rs")
        if name in {"name.rs", "packet.rs"}:
            return "DNS data model and wire codec", "rgbdns"
        if name in {"zone.rs", "server.rs", "cdb.rs"}:
            return "Authoritative service", "rgbdns"
        if name in {"client.rs", "dnscache_config.rs"}:
            return "Resolution and recursion", "rgbdns"
        if name in {"axfr.rs", "transport.rs"}:
            return "Transport and zone transfer", "rgbdns"
        if name in {"conf.rs", "multilog.rs", "setuidgid.rs", "tai64.rs"}:
            return "Operations and supervision", "rgbdns"
        if name in {"pick.rs", "rbl.rs", "wall.rs", "special.rs"}:
            return "Specialized responders", "rgbdns"
        return "Rust library", "rgbdns"
    if parts and parts[0] in {"tests", "benches", "examples"}:
        return "Tests and performance", None
    if parts and parts[0] == "docs":
        return "Documentation", None
    if parts and parts[0] == "scripts":
        return "Developer scripts", None
    if parts and parts[0] == ".github":
        return "Project automation", None
    return "Repository and build", None


```
