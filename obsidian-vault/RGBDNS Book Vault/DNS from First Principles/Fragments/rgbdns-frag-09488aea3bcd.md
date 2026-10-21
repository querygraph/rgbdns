---
type: "code-fragment"
fragment_id: "rgbdns-frag-09488aea3bcd"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "migrate_plugin_list"
kind: "def"
start_line: 603
end_line: 620
---

# migrate_plugin_list

- Fragment ID: `rgbdns-frag-09488aea3bcd`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 603-620
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-09488aea3bcd", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 603, "endLine": 620}
```

## Excerpt

<span id="rgbdns-frag-09488aea3bcd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-09488aea3bcd: def migrate_plugin_list

```python
def migrate_plugin_list(previous: list[str] | None) -> list[str]:
    """Carry an existing enablement list onto FirstPair Reader.

    A clean output seeds an empty list (FirstPair: bundled plugins are disabled
    until the reader enables them). A rebuild keeps the existing list and its
    order, replacing a legacy reader id with ``firstpair-reader`` in place.
    """
    if previous is None:
        return []
    migrated: list[str] = []
    for plugin_id in previous:
        if plugin_id in LEGACY_PLUGIN_IDS:
            plugin_id = READER_PLUGIN_ID
        if plugin_id not in migrated:
            migrated.append(plugin_id)
    return migrated


```
