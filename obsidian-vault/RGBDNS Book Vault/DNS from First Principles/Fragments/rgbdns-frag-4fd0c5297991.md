---
type: "code-fragment"
fragment_id: "rgbdns-frag-4fd0c5297991"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "clean_name"
kind: "def"
start_line: 126
end_line: 131
---

# clean_name

- Fragment ID: `rgbdns-frag-4fd0c5297991`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 126-131
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-4fd0c5297991", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 126, "endLine": 131}
```

## Excerpt

<span id="rgbdns-frag-4fd0c5297991" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4fd0c5297991: def clean_name

```python
def clean_name(value: str, limit: int = 100) -> str:
    value = re.sub(r'[\\/:*?"<>|#^[\]]+', " ", value)
    value = re.sub(r"\s+", " ", value).strip(" .")
    return (value or "Untitled")[:limit].rstrip()


```
