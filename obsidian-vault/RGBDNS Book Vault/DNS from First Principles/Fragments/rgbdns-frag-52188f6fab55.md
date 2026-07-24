---
type: "code-fragment"
fragment_id: "rgbdns-frag-52188f6fab55"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_index"
kind: "def"
start_line: 466
end_line: 472
---

# render_index

- Fragment ID: `rgbdns-frag-52188f6fab55`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 466-472
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-52188f6fab55", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-52188f6fab55: def render_index", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 466, "endLine": 472}
```

## Excerpt

<span id="rgbdns-frag-52188f6fab55" class="rgbdns-fragment-target"></span>
### rgbdns-frag-52188f6fab55: def render_index

```python
def render_index(title: str, items: Iterable[tuple[str, str]]) -> str:
    lines = [f"# {title}", ""]
    for label, path in sorted(items):
        lines.append(f"- {wiki(path, label)}")
    return "\n".join(lines)


```
