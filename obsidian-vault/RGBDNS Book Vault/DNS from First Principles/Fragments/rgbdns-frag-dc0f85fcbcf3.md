---
type: "code-fragment"
fragment_id: "rgbdns-frag-dc0f85fcbcf3"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "fragment_id"
kind: "def"
start_line: 267
end_line: 271
---

# fragment_id

- Fragment ID: `rgbdns-frag-dc0f85fcbcf3`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 267-271
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-dc0f85fcbcf3", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-dc0f85fcbcf3: def fragment_id", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 267, "endLine": 271}
```

## Excerpt

<span id="rgbdns-frag-dc0f85fcbcf3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-dc0f85fcbcf3: def fragment_id

```python
def fragment_id(path: str, start: int, end: int, symbol: str) -> str:
    raw = f"{path}:{start}:{end}:{symbol}".encode("utf-8")
    return f"rgbdns-frag-{hashlib.sha1(raw).hexdigest()[:12]}"


```
