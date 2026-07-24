---
type: "code-fragment"
fragment_id: "rgbdns-frag-858328602ac9"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "slug"
kind: "def"
start_line: 124
end_line: 128
---

# slug

- Fragment ID: `rgbdns-frag-858328602ac9`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 124-128
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-858328602ac9", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-858328602ac9: def slug", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 124, "endLine": 128}
```

## Excerpt

<span id="rgbdns-frag-858328602ac9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-858328602ac9: def slug

```python
def slug(value: str, limit: int = 80) -> str:
    value = re.sub(r"[^A-Za-z0-9_.-]+", "-", value).strip("-")
    return (value or "item")[:limit].strip("-")


```
