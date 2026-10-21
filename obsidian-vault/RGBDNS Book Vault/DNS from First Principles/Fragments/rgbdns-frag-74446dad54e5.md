---
type: "code-fragment"
fragment_id: "rgbdns-frag-74446dad54e5"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "load_json"
kind: "def"
start_line: 64
end_line: 67
---

# load_json

- Fragment ID: `rgbdns-frag-74446dad54e5`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 64-67
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-74446dad54e5", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 64, "endLine": 67}
```

## Excerpt

<span id="rgbdns-frag-74446dad54e5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-74446dad54e5: def load_json

```python
def load_json(path: Path) -> object:
    return json.loads(path.read_text(encoding="utf-8"))


```
