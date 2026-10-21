---
type: "code-fragment"
fragment_id: "rgbdns-frag-a33046fca8a8"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "code_note_path"
kind: "def"
start_line: 280
end_line: 284
---

# code_note_path

- Fragment ID: `rgbdns-frag-a33046fca8a8`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 280-284
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-a33046fca8a8", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 280, "endLine": 284}
```

## Excerpt

<span id="rgbdns-frag-a33046fca8a8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a33046fca8a8: def code_note_path

```python
def code_note_path(path: str) -> str:
    safe = "/".join(clean_name(part, 120) for part in PurePosixPath(path).parts)
    return f"{VAULT_BOOK}/Code/{safe}.source"


```
