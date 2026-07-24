---
type: "code-fragment"
fragment_id: "rgbdns-frag-7ed2d0aadd41"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "SourceFile"
kind: "class"
start_line: 82
end_line: 94
---

# SourceFile

- Fragment ID: `rgbdns-frag-7ed2d0aadd41`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 82-94
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-7ed2d0aadd41", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-7ed2d0aadd41: class SourceFile", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 82, "endLine": 94}
```

## Excerpt

<span id="rgbdns-frag-7ed2d0aadd41" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7ed2d0aadd41: class SourceFile

```python
class SourceFile:
    path: str
    absolute: Path
    note_path: str
    language: str
    subsystem: str
    crate: str | None
    lines: list[str]
    summary: str
    fragments: list["Fragment"] = field(default_factory=list)


@dataclass(slots=True)
```
