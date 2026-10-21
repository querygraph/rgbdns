---
type: "code-fragment"
fragment_id: "rgbdns-frag-c064dbf43539"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_chapter"
kind: "def"
start_line: 433
end_line: 449
---

# render_chapter

- Fragment ID: `rgbdns-frag-c064dbf43539`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 433-449
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-c064dbf43539", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 433, "endLine": 449}
```

## Excerpt

<span id="rgbdns-frag-c064dbf43539" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c064dbf43539: def render_chapter

```python
def render_chapter(title: str, text: str, fragments: list[Fragment]) -> str:
    related = choose_chapter_fragments(title, fragments)
    body = [text, "", "## Generated Code Fragment Index", ""]
    if related:
        body.append(
            "Each card names an rgbdns fragment. In FirstPair Reader, Show code lists its "
            "numbered lines and Open file opens the collocated code note with those lines marked."
        )
        body.append("")
        for fragment in related:
            body.append(render_fragment_block(fragment))
            body.append("")
    else:
        body.append("No generated fragments were matched for this chapter.")
    return "\n".join(body)


```
