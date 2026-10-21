---
type: "code-fragment"
fragment_id: "rgbdns-frag-40cf69fdf4d3"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_fragment_block"
kind: "def"
start_line: 422
end_line: 432
---

# render_fragment_block

- Fragment ID: `rgbdns-frag-40cf69fdf4d3`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 422-432
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-40cf69fdf4d3", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 422, "endLine": 432}
```

## Excerpt

<span id="rgbdns-frag-40cf69fdf4d3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-40cf69fdf4d3: def render_fragment_block

```python
def render_fragment_block(fragment: Fragment) -> str:
    payload = {
        "id": fragment.id,
        "codeNote": fragment.code_note,
        "sourcePath": fragment.source_path,
        "startLine": fragment.start_line,
        "endLine": fragment.end_line,
    }
    return "```rgbdns-fragment\n" + json.dumps(payload, ensure_ascii=True) + "\n```"


```
