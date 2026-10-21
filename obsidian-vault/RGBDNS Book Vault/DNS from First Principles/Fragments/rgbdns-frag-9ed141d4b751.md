---
type: "code-fragment"
fragment_id: "rgbdns-frag-9ed141d4b751"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "slug"
kind: "def"
start_line: 132
end_line: 136
---

# slug

- Fragment ID: `rgbdns-frag-9ed141d4b751`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 132-136
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-9ed141d4b751", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 132, "endLine": 136}
```

## Excerpt

<span id="rgbdns-frag-9ed141d4b751" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9ed141d4b751: def slug

```python
def slug(value: str, limit: int = 80) -> str:
    value = re.sub(r"[^A-Za-z0-9_.-]+", "-", value).strip("-")
    return (value or "item")[:limit].strip("-")


```
