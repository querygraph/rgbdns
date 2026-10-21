---
type: "code-fragment"
fragment_id: "rgbdns-frag-f30c6d208bb9"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "wiki"
kind: "def"
start_line: 158
end_line: 162
---

# wiki

- Fragment ID: `rgbdns-frag-f30c6d208bb9`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 158-162
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-f30c6d208bb9", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 158, "endLine": 162}
```

## Excerpt

<span id="rgbdns-frag-f30c6d208bb9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f30c6d208bb9: def wiki

```python
def wiki(path: str, label: str | None = None) -> str:
    target = path[:-3] if path.endswith(".md") else path
    return f"[[{target}|{label}]]" if label else f"[[{target}]]"


```
