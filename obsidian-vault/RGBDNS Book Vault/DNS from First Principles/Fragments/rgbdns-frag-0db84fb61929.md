---
type: "code-fragment"
fragment_id: "rgbdns-frag-0db84fb61929"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "yaml_value"
kind: "def"
start_line: 137
end_line: 140
---

# yaml_value

- Fragment ID: `rgbdns-frag-0db84fb61929`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 137-140
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-0db84fb61929", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 137, "endLine": 140}
```

## Excerpt

<span id="rgbdns-frag-0db84fb61929" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0db84fb61929: def yaml_value

```python
def yaml_value(value: object) -> str:
    return json.dumps(value, ensure_ascii=True)


```
