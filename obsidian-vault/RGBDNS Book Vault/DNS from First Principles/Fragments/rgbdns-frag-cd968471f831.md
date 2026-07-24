---
type: "code-fragment"
fragment_id: "rgbdns-frag-cd968471f831"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "code_note_path"
kind: "def"
start_line: 262
end_line: 266
---

# code_note_path

- Fragment ID: `rgbdns-frag-cd968471f831`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 262-266
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-cd968471f831", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-cd968471f831: def code_note_path", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 262, "endLine": 266}
```

## Excerpt

<span id="rgbdns-frag-cd968471f831" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cd968471f831: def code_note_path

```python
def code_note_path(path: str) -> str:
    safe = "/".join(clean_name(part, 120) for part in PurePosixPath(path).parts)
    return f"{VAULT_BOOK}/Code/{safe}.source"


```
