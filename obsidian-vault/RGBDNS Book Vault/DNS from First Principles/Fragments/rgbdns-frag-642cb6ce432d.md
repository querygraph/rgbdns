---
type: "code-fragment"
fragment_id: "rgbdns-frag-642cb6ce432d"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "write_note"
kind: "def"
start_line: 168
end_line: 172
---

# write_note

- Fragment ID: `rgbdns-frag-642cb6ce432d`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 168-172
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-642cb6ce432d", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 168, "endLine": 172}
```

## Excerpt

<span id="rgbdns-frag-642cb6ce432d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-642cb6ce432d: def write_note

```python
def write_note(root: Path, path: str, metadata: dict[str, object], body: str) -> None:
    note = root / (path if path.endswith(".md") else f"{path}.md")
    write_text(note, f"{frontmatter(metadata)}\n\n{body}")


```
