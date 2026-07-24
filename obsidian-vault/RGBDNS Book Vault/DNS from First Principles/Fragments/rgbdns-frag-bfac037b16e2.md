---
type: "code-fragment"
fragment_id: "rgbdns-frag-bfac037b16e2"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "write_note"
kind: "def"
start_line: 160
end_line: 164
---

# write_note

- Fragment ID: `rgbdns-frag-bfac037b16e2`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 160-164
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-bfac037b16e2", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-bfac037b16e2: def write_note", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 160, "endLine": 164}
```

## Excerpt

<span id="rgbdns-frag-bfac037b16e2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bfac037b16e2: def write_note

```python
def write_note(root: Path, path: str, metadata: dict[str, object], body: str) -> None:
    note = root / (path if path.endswith(".md") else f"{path}.md")
    write_text(note, f"{frontmatter(metadata)}\n\n{body}")


```
