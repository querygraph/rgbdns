---
type: "code-fragment"
fragment_id: "rgbdns-frag-bcaecfe813fa"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "load_json"
kind: "def"
start_line: 55
end_line: 58
---

# load_json

- Fragment ID: `rgbdns-frag-bcaecfe813fa`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 55-58
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-bcaecfe813fa", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "heading": "rgbdns-frag-bcaecfe813fa: def load_json", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 55, "endLine": 58}
```

## Excerpt

<span id="rgbdns-frag-bcaecfe813fa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bcaecfe813fa: def load_json

```python
def load_json(path: Path) -> object:
    return json.loads(path.read_text(encoding="utf-8"))


```
