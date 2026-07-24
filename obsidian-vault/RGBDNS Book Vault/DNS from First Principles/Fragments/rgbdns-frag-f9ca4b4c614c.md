---
type: "code-fragment"
fragment_id: "rgbdns-frag-f9ca4b4c614c"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "target_exists"
kind: "def"
start_line: 59
end_line: 71
---

# target_exists

- Fragment ID: `rgbdns-frag-f9ca4b4c614c`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 59-71
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-f9ca4b4c614c", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "heading": "rgbdns-frag-f9ca4b4c614c: def target_exists", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 59, "endLine": 71}
```

## Excerpt

<span id="rgbdns-frag-f9ca4b4c614c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f9ca4b4c614c: def target_exists

```python
def target_exists(target: str, notes: set[str], files: set[str]) -> bool:
    target = target.split("|", 1)[0].split("#", 1)[0]
    if not target:
        return True
    if target in notes:
        return True
    if target.endswith(".md") and target[:-3] in notes:
        return True
    if target in files:
        return True
    return False


```
