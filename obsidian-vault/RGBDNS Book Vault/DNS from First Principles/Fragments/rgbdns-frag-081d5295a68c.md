---
type: "code-fragment"
fragment_id: "rgbdns-frag-081d5295a68c"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "write_text"
kind: "def"
start_line: 155
end_line: 159
---

# write_text

- Fragment ID: `rgbdns-frag-081d5295a68c`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 155-159
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-081d5295a68c", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-081d5295a68c: def write_text", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 155, "endLine": 159}
```

## Excerpt

<span id="rgbdns-frag-081d5295a68c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-081d5295a68c: def write_text

```python
def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text.rstrip() + "\n", encoding="utf-8")


```
