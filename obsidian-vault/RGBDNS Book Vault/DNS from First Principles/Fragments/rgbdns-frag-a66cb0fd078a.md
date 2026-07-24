---
type: "code-fragment"
fragment_id: "rgbdns-frag-a66cb0fd078a"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_fragment_block"
kind: "def"
start_line: 388
end_line: 399
---

# render_fragment_block

- Fragment ID: `rgbdns-frag-a66cb0fd078a`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 388-399
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-a66cb0fd078a", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-a66cb0fd078a: def render_fragment_block", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 388, "endLine": 399}
```

## Excerpt

<span id="rgbdns-frag-a66cb0fd078a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a66cb0fd078a: def render_fragment_block

```python
def render_fragment_block(fragment: Fragment) -> str:
    payload = {
        "id": fragment.id,
        "codeNote": fragment.code_note,
        "heading": fragment.heading,
        "sourcePath": fragment.source_path,
        "startLine": fragment.start_line,
        "endLine": fragment.end_line,
    }
    return "```rgbdns-fragment\n" + json.dumps(payload, ensure_ascii=True) + "\n```"


```
