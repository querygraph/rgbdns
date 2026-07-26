---
type: "code-fragment"
fragment_id: "rgbdns-frag-3f8c0be55fcf"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_index"
kind: "def"
start_line: 482
end_line: 488
---

# render_index

- Fragment ID: `rgbdns-frag-3f8c0be55fcf`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 482-488
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-3f8c0be55fcf", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-3f8c0be55fcf: def render_index", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 482, "endLine": 488}
```

## Excerpt

<span id="rgbdns-frag-3f8c0be55fcf" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3f8c0be55fcf: def render_index

```python
def render_index(title: str, items: Iterable[tuple[str, str]]) -> str:
    lines = [f"# {title}", ""]
    for label, path in sorted(items):
        lines.append(f"- {wiki(path, label)}")
    return "\n".join(lines)


```
