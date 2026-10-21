---
type: "code-fragment"
fragment_id: "rgbdns-frag-66fe076b7df1"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "read_text"
kind: "def"
start_line: 262
end_line: 279
---

# read_text

- Fragment ID: `rgbdns-frag-66fe076b7df1`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 262-279
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-66fe076b7df1", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 262, "endLine": 279}
```

## Excerpt

<span id="rgbdns-frag-66fe076b7df1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-66fe076b7df1: def read_text

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
