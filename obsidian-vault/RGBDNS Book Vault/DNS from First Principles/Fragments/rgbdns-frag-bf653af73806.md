---
type: "code-fragment"
fragment_id: "rgbdns-frag-bf653af73806"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "read_text"
kind: "def"
start_line: 244
end_line: 261
---

# read_text

- Fragment ID: `rgbdns-frag-bf653af73806`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 244-261
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-bf653af73806", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-bf653af73806: def read_text", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 244, "endLine": 261}
```

## Excerpt

<span id="rgbdns-frag-bf653af73806" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bf653af73806: def read_text

```python
def read_text(path: Path) -> str | None:
    if path.stat().st_size > 1_500_000:
        return None
    try:
        data = path.read_bytes()
    except OSError:
        return None
    if b"\x00" in data:
        return None
    try:
        return data.decode("utf-8")
    except UnicodeDecodeError:
        try:
            return data.decode("utf-8", errors="replace")
        except Exception:
            return None


```
