---
type: "code-fragment"
fragment_id: "rgbdns-frag-40e10e268998"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "note_key"
kind: "def"
start_line: 51
end_line: 54
---

# note_key

- Fragment ID: `rgbdns-frag-40e10e268998`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 51-54
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-40e10e268998", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "heading": "rgbdns-frag-40e10e268998: def note_key", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 51, "endLine": 54}
```

## Excerpt

<span id="rgbdns-frag-40e10e268998" class="rgbdns-fragment-target"></span>
### rgbdns-frag-40e10e268998: def note_key

```python
def note_key(path: Path, root: Path) -> str:
    return path.relative_to(root).with_suffix("").as_posix()


```
