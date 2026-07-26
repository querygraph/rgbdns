---
type: "code-fragment"
fragment_id: "rgbdns-frag-21b62d47f194"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_source_file"
kind: "def"
start_line: 430
end_line: 453
---

# render_source_file

- Fragment ID: `rgbdns-frag-21b62d47f194`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 430-453
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-21b62d47f194", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-21b62d47f194: def render_source_file", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 430, "endLine": 453}
```

## Excerpt

<span id="rgbdns-frag-21b62d47f194" class="rgbdns-fragment-target"></span>
### rgbdns-frag-21b62d47f194: def render_source_file

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
    lines.extend(["", "## Full Source", "", f"```{source.language}"])
    lines.extend(source.lines)
    lines.append("```")
    return "\n".join(lines)


```
