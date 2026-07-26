---
type: "code-fragment"
fragment_id: "rgbdns-frag-8c6f87b3e8be"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_chapter"
kind: "def"
start_line: 416
end_line: 429
---

# render_chapter

- Fragment ID: `rgbdns-frag-8c6f87b3e8be`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 416-429
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-8c6f87b3e8be", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-8c6f87b3e8be: def render_chapter", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 416, "endLine": 429}
```

## Excerpt

<span id="rgbdns-frag-8c6f87b3e8be" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8c6f87b3e8be: def render_chapter

```python
def render_chapter(title: str, text: str, fragments: list[Fragment]) -> str:
    related = choose_chapter_fragments(title, fragments)
    body = [text, "", "## Generated Code Fragment Index", ""]
    if related:
        body.append("These generated links open the collocated rgbdns codebase notes.")
        body.append("")
        for fragment in related:
            body.append(render_fragment_block(fragment))
            body.append("")
    else:
        body.append("No generated fragments were matched for this chapter.")
    return "\n".join(body)


```
