---
type: "code-fragment"
fragment_id: "rgbdns-frag-acdb0659eaed"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "write_text"
kind: "def"
start_line: 163
end_line: 167
---

# write_text

- Fragment ID: `rgbdns-frag-acdb0659eaed`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 163-167
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-acdb0659eaed", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 163, "endLine": 167}
```

## Excerpt

<span id="rgbdns-frag-acdb0659eaed" class="rgbdns-fragment-target"></span>
### rgbdns-frag-acdb0659eaed: def write_text

```python
def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text.rstrip() + "\n", encoding="utf-8")


```
