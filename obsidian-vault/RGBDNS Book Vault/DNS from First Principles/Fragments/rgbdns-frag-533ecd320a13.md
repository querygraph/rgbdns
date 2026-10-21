---
type: "code-fragment"
fragment_id: "rgbdns-frag-533ecd320a13"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_index"
kind: "def"
start_line: 507
end_line: 513
---

# render_index

- Fragment ID: `rgbdns-frag-533ecd320a13`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 507-513
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-533ecd320a13", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 507, "endLine": 513}
```

## Excerpt

<span id="rgbdns-frag-533ecd320a13" class="rgbdns-fragment-target"></span>
### rgbdns-frag-533ecd320a13: def render_index

```python
def render_index(title: str, items: Iterable[tuple[str, str]]) -> str:
    lines = [f"# {title}", ""]
    for label, path in sorted(items):
        lines.append(f"- {wiki(path, label)}")
    return "\n".join(lines)


```
