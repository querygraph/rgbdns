---
type: "code-fragment"
fragment_id: "rgbdns-frag-a1ba6e09005f"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "fragment_id"
kind: "def"
start_line: 285
end_line: 289
---

# fragment_id

- Fragment ID: `rgbdns-frag-a1ba6e09005f`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 285-289
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-a1ba6e09005f", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 285, "endLine": 289}
```

## Excerpt

<span id="rgbdns-frag-a1ba6e09005f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a1ba6e09005f: def fragment_id

```python
def fragment_id(path: str, start: int, end: int, symbol: str) -> str:
    raw = f"{path}:{start}:{end}:{symbol}".encode("utf-8")
    return f"rgbdns-frag-{hashlib.sha1(raw).hexdigest()[:12]}"


```
