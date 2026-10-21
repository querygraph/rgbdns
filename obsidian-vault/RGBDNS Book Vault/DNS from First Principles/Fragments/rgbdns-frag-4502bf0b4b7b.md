---
type: "code-fragment"
fragment_id: "rgbdns-frag-4502bf0b4b7b"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_source_file"
kind: "def"
start_line: 450
end_line: 477
---

# render_source_file

- Fragment ID: `rgbdns-frag-4502bf0b4b7b`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 450-477
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-4502bf0b4b7b", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 450, "endLine": 477}
```

## Excerpt

<span id="rgbdns-frag-4502bf0b4b7b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4502bf0b4b7b: def render_source_file

```python
def render_source_file(source: SourceFile) -> str:
    lines = [
        f"# {source.path}",
        "",
        f"- Subsystem: [[{VAULT_BOOK}/Subsystems/{clean_name(source.subsystem)}|{source.subsystem}]]",
    ]
    if source.crate:
        lines.append(f"- Component: [[{VAULT_BOOK}/Components/{source.crate}|{source.crate}]]")
    lines.extend([
        f"- Source path: `{source.path}`",
        f"- Lines: {len(source.lines)}",
        f"- Summary: {source.summary}",
        "",
        "## Extracted Fragments",
        "",
    ])
    for fragment in source.fragments[:80]:
        lines.append(f"- [[{fragment.note_path}|{fragment.symbol}]]: lines {fragment.start_line}-{fragment.end_line}")
    # Verbatim: fence_for picks an outer fence longer than any inside, which
    # Obsidian and FirstPair Reader 2 both honour (CommonMark).
    shown = source.lines
    fence = fence_for(shown)
    lines.extend(["", "## Full Source", "", f"{fence}{source.language}"])
    lines.extend(shown)
    lines.append(fence)
    return "\n".join(lines)


```
