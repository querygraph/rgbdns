---
type: "code-fragment"
fragment_id: "rgbdns-frag-4ce63a83ca29"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "strip_fenced_code"
kind: "def"
start_line: 18
end_line: 29
---

# strip_fenced_code

- Fragment ID: `rgbdns-frag-4ce63a83ca29`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 18-29
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-4ce63a83ca29", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "heading": "rgbdns-frag-4ce63a83ca29: def strip_fenced_code", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 18, "endLine": 29}
```

## Excerpt

<span id="rgbdns-frag-4ce63a83ca29" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4ce63a83ca29: def strip_fenced_code

```python
def strip_fenced_code(text: str) -> str:
    lines: list[str] = []
    in_fence = False
    for line in text.splitlines():
        if FENCE_RE.match(line):
            in_fence = not in_fence
            lines.append("")
            continue
        lines.append("" if in_fence else line)
    return "\n".join(lines)


```
