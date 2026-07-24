---
type: "code-fragment"
fragment_id: "rgbdns-frag-ea4248e89b2b"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "summary_for"
kind: "def"
start_line: 217
end_line: 230
---

# summary_for

- Fragment ID: `rgbdns-frag-ea4248e89b2b`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 217-230
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-ea4248e89b2b", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-ea4248e89b2b: def summary_for", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 217, "endLine": 230}
```

## Excerpt

<span id="rgbdns-frag-ea4248e89b2b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ea4248e89b2b: def summary_for

```python
def summary_for(path: str, subsystem: str, lines: list[str]) -> str:
    if path.endswith("Cargo.toml"):
        return f"Cargo manifest for {subsystem}."
    for line in lines[:80]:
        stripped = line.strip()
        if stripped.startswith("//!") or stripped.startswith("///"):
            return stripped.lstrip("/! ").strip()
        if stripped.startswith('"""') and len(stripped) > 3:
            return stripped.strip('" ')
        if stripped.startswith("# "):
            return stripped.lstrip("# ").strip()
    return f"Source file in the {subsystem} subsystem."


```
