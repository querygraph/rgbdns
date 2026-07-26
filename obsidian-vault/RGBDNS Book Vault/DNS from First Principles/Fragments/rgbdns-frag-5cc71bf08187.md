---
type: "code-fragment"
fragment_id: "rgbdns-frag-5cc71bf08187"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_fragment_block"
kind: "def"
start_line: 404
end_line: 415
---

# render_fragment_block

- Fragment ID: `rgbdns-frag-5cc71bf08187`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 404-415
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-5cc71bf08187", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-5cc71bf08187: def render_fragment_block", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 404, "endLine": 415}
```

## Excerpt

<span id="rgbdns-frag-5cc71bf08187" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5cc71bf08187: def render_fragment_block

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
