---
type: "code-fragment"
fragment_id: "rgbdns-frag-2051053e2baa"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "SourceFile"
kind: "class"
start_line: 90
end_line: 102
---

# SourceFile

- Fragment ID: `rgbdns-frag-2051053e2baa`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 90-102
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-2051053e2baa", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 90, "endLine": 102}
```

## Excerpt

<span id="rgbdns-frag-2051053e2baa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2051053e2baa: class SourceFile

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
