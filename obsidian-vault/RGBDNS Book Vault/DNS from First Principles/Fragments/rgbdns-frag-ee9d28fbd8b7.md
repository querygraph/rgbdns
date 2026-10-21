---
type: "code-fragment"
fragment_id: "rgbdns-frag-ee9d28fbd8b7"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "reader_excerpt_lines"
kind: "def"
start_line: 68
end_line: 91
---

# reader_excerpt_lines

- Fragment ID: `rgbdns-frag-ee9d28fbd8b7`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 68-91
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-ee9d28fbd8b7", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 68, "endLine": 91}
```

## Excerpt

<span id="rgbdns-frag-ee9d28fbd8b7" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ee9d28fbd8b7: def reader_excerpt_lines

```python
def reader_excerpt_lines(text: str) -> list[str] | None:
    """Return the code lines FirstPair Reader 2 numbers for a code note.

    Mirrors the Reader's fencedLines: the first fence after "## Full Source"
    closes only at a fence of the same character at least as long (CommonMark).
    """
    full = text.find("## Full Source")
    if full < 0:
        return None
    lines = text[full:].split("\n")
    opening = None
    for index, line in enumerate(lines):
        match = re.match(r" {0,3}(`{3,}|~{3,})", line)
        if not match:
            continue
        if opening is None:
            opening = (index, match.group(1))
            continue
        closing = re.match(r" {0,3}(`+|~+)\s*$", line)
        if closing and closing.group(1)[0] == opening[1][0] and len(closing.group(1)) >= len(opening[1]):
            return lines[opening[0] + 1:index]
    return None if opening is None else lines[opening[0] + 1:]


```
