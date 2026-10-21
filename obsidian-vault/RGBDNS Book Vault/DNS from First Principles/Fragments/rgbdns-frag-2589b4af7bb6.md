---
type: "code-fragment"
fragment_id: "rgbdns-frag-2589b4af7bb6"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "fence_for"
kind: "def"
start_line: 173
end_line: 182
---

# fence_for

- Fragment ID: `rgbdns-frag-2589b4af7bb6`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 173-182
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-2589b4af7bb6", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 173, "endLine": 182}
```

## Excerpt

<span id="rgbdns-frag-2589b4af7bb6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2589b4af7bb6: def fence_for

```python
def fence_for(lines: Iterable[str]) -> str:
    """Return a backtick fence longer than any fence-like run inside ``lines``."""
    longest = 0
    for line in lines:
        match = re.match(r" {0,3}(`{3,})", line)
        if match:
            longest = max(longest, len(match.group(1)))
    return "`" * max(3, longest + 1)


```
