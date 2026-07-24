---
type: "code-fragment"
fragment_id: "rgbdns-frag-b08d61618d58"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "wiki"
kind: "def"
start_line: 150
end_line: 154
---

# wiki

- Fragment ID: `rgbdns-frag-b08d61618d58`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 150-154
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-b08d61618d58", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-b08d61618d58: def wiki", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 150, "endLine": 154}
```

## Excerpt

<span id="rgbdns-frag-b08d61618d58" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b08d61618d58: def wiki

```python
def wiki(path: str, label: str | None = None) -> str:
    target = path[:-3] if path.endswith(".md") else path
    return f"[[{target}|{label}]]" if label else f"[[{target}]]"


```
