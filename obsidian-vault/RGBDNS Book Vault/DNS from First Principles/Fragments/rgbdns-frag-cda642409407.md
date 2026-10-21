---
type: "code-fragment"
fragment_id: "rgbdns-frag-cda642409407"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "read_plugin_list"
kind: "def"
start_line: 589
end_line: 602
---

# read_plugin_list

- Fragment ID: `rgbdns-frag-cda642409407`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 589-602
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-cda642409407", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 589, "endLine": 602}
```

## Excerpt

<span id="rgbdns-frag-cda642409407" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cda642409407: def read_plugin_list

```python
def read_plugin_list(output: Path) -> list[str] | None:
    """Return an existing vault's community plugin list, or None for a clean output."""
    path = output / ".obsidian" / "community-plugins.json"
    if not path.is_file():
        return None
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None
    if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
        return None
    return value


```
