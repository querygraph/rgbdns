---
type: "code-fragment"
fragment_id: "rgbdns-frag-ca3cce7168e8"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "manuscript_chapters"
kind: "def"
start_line: 360
end_line: 388
---

# manuscript_chapters

- Fragment ID: `rgbdns-frag-ca3cce7168e8`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 360-388
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-ca3cce7168e8", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 360, "endLine": 388}
```

## Excerpt

<span id="rgbdns-frag-ca3cce7168e8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ca3cce7168e8: def manuscript_chapters

```python
def manuscript_chapters() -> list[tuple[str, str, str]]:
    """Split the canonical single-file manuscript into stable Obsidian notes."""
    text = MANUSCRIPT.read_text(encoding="utf-8")
    lines = text.splitlines()
    starts: list[int] = []
    fence: str | None = None
    for index, line in enumerate(lines):
        stripped = line.lstrip()
        marker = stripped[:3]
        if marker in {"```", "~~~"}:
            if fence is None:
                fence = marker
            elif marker == fence:
                fence = None
            continue
        if fence is None and line.startswith("# "):
            starts.append(index)

    chapters: list[tuple[str, str, str]] = []
    for index, start in enumerate(starts, start=1):
        end = starts[index] if index < len(starts) else len(lines)
        chunk = "\n".join(lines[start:end]).rstrip()
        first = chunk.splitlines()[0]
        title = re.sub(r"\s+\{[^}]+\}\s*$", "", first[2:]).strip()
        note = f"{VAULT_BOOK}/Chapters/{index:02d}-{slug(title, 72)}"
        chapters.append((title, note, chunk.rstrip()))
    return chapters


```
