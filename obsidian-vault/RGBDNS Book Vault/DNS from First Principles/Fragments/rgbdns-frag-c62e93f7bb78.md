---
type: "code-fragment"
fragment_id: "rgbdns-frag-c62e93f7bb78"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "target_exists"
kind: "def"
start_line: 184
end_line: 196
---

# target_exists

- Fragment ID: `rgbdns-frag-c62e93f7bb78`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 184-196
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-c62e93f7bb78", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 184, "endLine": 196}
```

## Excerpt

<span id="rgbdns-frag-c62e93f7bb78" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c62e93f7bb78: def target_exists

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
