---
type: "code-fragment"
fragment_id: "rgbdns-frag-f4e9558a39ba"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "yaml_value"
kind: "def"
start_line: 129
end_line: 132
---

# yaml_value

- Fragment ID: `rgbdns-frag-f4e9558a39ba`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 129-132
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-f4e9558a39ba", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-f4e9558a39ba: def yaml_value", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 129, "endLine": 132}
```

## Excerpt

<span id="rgbdns-frag-f4e9558a39ba" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f4e9558a39ba: def yaml_value

```python
def yaml_value(value: object) -> str:
    return json.dumps(value, ensure_ascii=True)


```
