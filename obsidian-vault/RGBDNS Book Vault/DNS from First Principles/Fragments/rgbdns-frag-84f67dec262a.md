---
type: "code-fragment"
fragment_id: "rgbdns-frag-84f67dec262a"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "should_include"
kind: "def"
start_line: 249
end_line: 261
---

# should_include

- Fragment ID: `rgbdns-frag-84f67dec262a`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 249-261
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-84f67dec262a", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 249, "endLine": 261}
```

## Excerpt

<span id="rgbdns-frag-84f67dec262a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-84f67dec262a: def should_include

```python
def should_include(path: Path, root: Path) -> bool:
    rel = path.relative_to(root)
    if any(part in SKIP_DIRS for part in rel.parts):
        return False
    if not path.is_file():
        return False
    if path.name in TEXT_NAMES:
        return True
    if path.suffix.lower() in TEXT_SUFFIXES:
        return True
    return False


```
