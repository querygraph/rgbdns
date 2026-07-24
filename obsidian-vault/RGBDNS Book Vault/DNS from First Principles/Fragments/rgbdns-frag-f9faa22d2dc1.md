---
type: "code-fragment"
fragment_id: "rgbdns-frag-f9faa22d2dc1"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "clean_name"
kind: "def"
start_line: 118
end_line: 123
---

# clean_name

- Fragment ID: `rgbdns-frag-f9faa22d2dc1`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 118-123
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-f9faa22d2dc1", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-f9faa22d2dc1: def clean_name", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 118, "endLine": 123}
```

## Excerpt

<span id="rgbdns-frag-f9faa22d2dc1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f9faa22d2dc1: def clean_name

```python
def clean_name(value: str, limit: int = 100) -> str:
    value = re.sub(r'[\\/:*?"<>|#^[\]]+', " ", value)
    value = re.sub(r"\s+", " ", value).strip(" .")
    return (value or "Untitled")[:limit].rstrip()


```
