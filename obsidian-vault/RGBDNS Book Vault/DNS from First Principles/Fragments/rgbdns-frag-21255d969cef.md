---
type: "code-fragment"
fragment_id: "rgbdns-frag-21255d969cef"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "subsystem_for"
kind: "def"
start_line: 187
end_line: 216
---

# subsystem_for

- Fragment ID: `rgbdns-frag-21255d969cef`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 187-216
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-21255d969cef", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-21255d969cef: def subsystem_for", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 187, "endLine": 216}
```

## Excerpt

<span id="rgbdns-frag-21255d969cef" class="rgbdns-fragment-target"></span>
### rgbdns-frag-21255d969cef: def subsystem_for

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
