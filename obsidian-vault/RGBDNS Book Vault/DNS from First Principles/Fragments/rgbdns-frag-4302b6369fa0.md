---
type: "code-fragment"
fragment_id: "rgbdns-frag-4302b6369fa0"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_fragment_note"
kind: "def"
start_line: 438
end_line: 465
---

# render_fragment_note

- Fragment ID: `rgbdns-frag-4302b6369fa0`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 438-465
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-4302b6369fa0", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-4302b6369fa0: def render_fragment_note", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 438, "endLine": 465}
```

## Excerpt

<span id="rgbdns-frag-4302b6369fa0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4302b6369fa0: def render_fragment_note

```python
def render_fragment_note(fragment: Fragment, source: SourceFile) -> str:
    excerpt = source.lines[fragment.start_line - 1:fragment.end_line]
    lines = [
        f"# {fragment.symbol}",
        "",
        f"- Fragment ID: `{fragment.id}`",
        f"- Source file: [[{fragment.code_note}|{fragment.source_path}]]",
        f"- Lines: {fragment.start_line}-{fragment.end_line}",
        f"- Subsystem: [[{VAULT_BOOK}/Subsystems/{clean_name(fragment.subsystem)}|{fragment.subsystem}]]",
    ]
    if fragment.crate:
        lines.append(f"- Component: [[{VAULT_BOOK}/Components/{fragment.crate}|{fragment.crate}]]")
    lines.extend([
        "",
        render_fragment_block(fragment),
        "",
        "## Excerpt",
        "",
        f'<span id="{fragment.id}" class="rgbdns-fragment-target"></span>',
        f"### {fragment.heading}",
        "",
        f"```{fragment.language}",
        *excerpt,
        "```",
    ])
    return "\n".join(lines)


```
