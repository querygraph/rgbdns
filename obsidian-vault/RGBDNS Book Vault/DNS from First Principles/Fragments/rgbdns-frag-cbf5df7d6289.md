---
type: "code-fragment"
fragment_id: "rgbdns-frag-cbf5df7d6289"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "note_key"
kind: "def"
start_line: 60
end_line: 63
---

# note_key

- Fragment ID: `rgbdns-frag-cbf5df7d6289`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 60-63
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-cbf5df7d6289", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 60, "endLine": 63}
```

## Excerpt

<span id="rgbdns-frag-cbf5df7d6289" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cbf5df7d6289: def note_key

```python
def note_key(path: Path, root: Path) -> str:
    return path.relative_to(root).with_suffix("").as_posix()


```
