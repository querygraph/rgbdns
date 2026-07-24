---
type: "code-fragment"
fragment_id: "rgbdns-frag-14f680024c75"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "fragment_payloads"
kind: "def"
start_line: 30
end_line: 50
---

# fragment_payloads

- Fragment ID: `rgbdns-frag-14f680024c75`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 30-50
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-14f680024c75", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "heading": "rgbdns-frag-14f680024c75: def fragment_payloads", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 30, "endLine": 50}
```

## Excerpt

<span id="rgbdns-frag-14f680024c75" class="rgbdns-fragment-target"></span>
### rgbdns-frag-14f680024c75: def fragment_payloads

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
