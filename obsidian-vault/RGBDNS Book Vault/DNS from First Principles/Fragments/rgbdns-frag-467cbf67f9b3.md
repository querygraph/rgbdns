---
type: "code-fragment"
fragment_id: "rgbdns-frag-467cbf67f9b3"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "portable_root"
kind: "def"
start_line: 638
end_line: 646
---

# portable_root

- Fragment ID: `rgbdns-frag-467cbf67f9b3`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 638-646
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-467cbf67f9b3", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 638, "endLine": 646}
```

## Excerpt

<span id="rgbdns-frag-467cbf67f9b3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-467cbf67f9b3: def portable_root

```python
def portable_root(rgbdns_root: Path) -> str:
    """Describe the code checkout without recording a local absolute path."""
    try:
        relative = rgbdns_root.resolve().relative_to(REPO_ROOT.resolve()).as_posix()
    except ValueError:
        return "$RGBDNS_CODE_ROOT"
    return relative or "."


```
