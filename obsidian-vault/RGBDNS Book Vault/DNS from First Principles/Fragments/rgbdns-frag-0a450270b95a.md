---
type: "code-fragment"
fragment_id: "rgbdns-frag-0a450270b95a"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "manuscript_chapters"
kind: "def"
start_line: 342
end_line: 354
---

# manuscript_chapters

- Fragment ID: `rgbdns-frag-0a450270b95a`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 342-354
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-0a450270b95a", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-0a450270b95a: def manuscript_chapters", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 342, "endLine": 354}
```

## Excerpt

<span id="rgbdns-frag-0a450270b95a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0a450270b95a: def manuscript_chapters

```python
def manuscript_chapters() -> list[tuple[str, str, str]]:
    """Split the canonical single-file manuscript into stable Obsidian notes."""
    text = MANUSCRIPT.read_text(encoding="utf-8")
    chunks = re.split(r"(?m)(?=^# )", text)
    chapters: list[tuple[str, str, str]] = []
    for index, chunk in enumerate(filter(str.strip, chunks), start=1):
        first = chunk.splitlines()[0]
        title = re.sub(r"\s+\{[^}]+\}\s*$", "", first[2:]).strip()
        note = f"{VAULT_BOOK}/Chapters/{index:02d}-{slug(title, 72)}"
        chapters.append((title, note, chunk.rstrip()))
    return chapters


```
