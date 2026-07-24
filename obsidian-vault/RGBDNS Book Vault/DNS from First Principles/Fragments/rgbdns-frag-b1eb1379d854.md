---
type: "code-fragment"
fragment_id: "rgbdns-frag-b1eb1379d854"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "frontmatter"
kind: "def"
start_line: 133
end_line: 149
---

# frontmatter

- Fragment ID: `rgbdns-frag-b1eb1379d854`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 133-149
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-b1eb1379d854", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-b1eb1379d854: def frontmatter", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 133, "endLine": 149}
```

## Excerpt

<span id="rgbdns-frag-b1eb1379d854" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b1eb1379d854: def frontmatter

```python
def frontmatter(values: dict[str, object]) -> str:
    lines = ["---"]
    for key, value in values.items():
        if value is None:
            continue
        if isinstance(value, (list, tuple, set)):
            lines.append(f"{key}:")
            for item in value:
                lines.append(f"  - {yaml_value(item)}")
        elif isinstance(value, bool):
            lines.append(f"{key}: {'true' if value else 'false'}")
        else:
            lines.append(f"{key}: {yaml_value(value)}")
    lines.append("---")
    return "\n".join(lines)


```
