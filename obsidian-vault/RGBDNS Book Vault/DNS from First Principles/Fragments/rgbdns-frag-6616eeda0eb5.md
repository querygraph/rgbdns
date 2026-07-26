---
type: "code-fragment"
fragment_id: "rgbdns-frag-6616eeda0eb5"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "choose_chapter_fragments"
kind: "def"
start_line: 371
end_line: 403
---

# choose_chapter_fragments

- Fragment ID: `rgbdns-frag-6616eeda0eb5`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 371-403
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-6616eeda0eb5", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-6616eeda0eb5: def choose_chapter_fragments", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 371, "endLine": 403}
```

## Excerpt

<span id="rgbdns-frag-6616eeda0eb5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6616eeda0eb5: def choose_chapter_fragments

```python
def choose_chapter_fragments(title: str, fragments: list[Fragment], limit: int = 12) -> list[Fragment]:
    title_words = set(re.findall(r"[a-z0-9]+", title.lower()))
    mapping = {
        "name": {"src/name.rs"},
        "names": {"src/name.rs"},
        "packet": {"src/packet.rs"},
        "messages": {"src/packet.rs"},
        "wire": {"src/packet.rs", "src/transport.rs"},
        "authority": {"src/zone.rs", "src/server.rs", "src/cdb.rs"},
        "authoritative": {"src/zone.rs", "src/server.rs", "src/cdb.rs"},
        "recursion": {"src/dnscache_config.rs", "src/bin/dnscache.rs"},
        "dnssec": {"src/dnscache_config.rs", "src/bin/dnscache.rs"},
        "transfer": {"src/axfr.rs"},
        "client": {"src/client.rs"},
        "security": {"src/packet.rs", "tests/wire_security.rs", "tests/packet_properties.rs"},
        "performance": {"benches/dns_core.rs", "docs/performance.md"},
        "testing": {"tests/rfc_conformance.rs", "tests/wire_security.rs", "tests/packet_properties.rs"},
        "supervision": {"src/conf.rs", "src/multilog.rs", "src/setuidgid.rs"},
        "rust": {"src/name.rs", "src/packet.rs", "src/server.rs"},
        "codebase": {"src/lib.rs", "src/name.rs", "src/packet.rs", "src/zone.rs", "src/server.rs"},
        "cdb": {"src/cdb.rs", "src/zone.rs"},
        "transport": {"src/transport.rs", "src/client.rs", "src/axfr.rs"},
    }
    paths: set[str] = set()
    for word in title_words:
        paths.update(mapping.get(word, set()))
    if not paths:
        paths = {"src/lib.rs", "README.md"}
    selected = [fragment for fragment in fragments if fragment.source_path in paths]
    selected.sort(key=lambda fragment: (fragment.source_path, fragment.start_line))
    return selected[:limit]


```
