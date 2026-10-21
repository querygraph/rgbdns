---
type: "code-fragment"
fragment_id: "rgbdns-frag-1029d555bc2c"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "fragment_payloads"
kind: "def"
start_line: 39
end_line: 59
---

# fragment_payloads

- Fragment ID: `rgbdns-frag-1029d555bc2c`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 39-59
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-1029d555bc2c", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 39, "endLine": 59}
```

## Excerpt

<span id="rgbdns-frag-1029d555bc2c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1029d555bc2c: def fragment_payloads

```python
def fragment_payloads(text: str) -> list[str]:
    """Return rgbdns-fragment payloads that are not examples inside other fences."""
    payloads: list[str] = []
    lines = text.splitlines()
    index = 0
    in_other_fence = False
    while index < len(lines):
        stripped = lines[index].strip()
        if not in_other_fence and stripped == "```rgbdns-fragment":
            index += 1
            body: list[str] = []
            while index < len(lines) and lines[index].strip() != "```":
                body.append(lines[index])
                index += 1
            payloads.append("\n".join(body))
        elif stripped.startswith(("```", "~~~")):
            in_other_fence = not in_other_fence
        index += 1
    return payloads


```
