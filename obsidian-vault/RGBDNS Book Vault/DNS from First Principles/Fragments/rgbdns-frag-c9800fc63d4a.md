---
type: "code-fragment"
fragment_id: "rgbdns-frag-c9800fc63d4a"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "language_for"
kind: "def"
start_line: 165
end_line: 186
---

# language_for

- Fragment ID: `rgbdns-frag-c9800fc63d4a`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 165-186
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-c9800fc63d4a", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-c9800fc63d4a: def language_for", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 165, "endLine": 186}
```

## Excerpt

<span id="rgbdns-frag-c9800fc63d4a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c9800fc63d4a: def language_for

```python
def language_for(path: str) -> str:
    suffix = Path(path).suffix.lower()
    return {
        ".css": "css",
        ".feature": "gherkin",
        ".html": "html",
        ".js": "javascript",
        ".json": "json",
        ".md": "markdown",
        ".mjs": "javascript",
        ".proto": "protobuf",
        ".py": "python",
        ".rs": "rust",
        ".sh": "bash",
        ".toml": "toml",
        ".ts": "typescript",
        ".tsx": "tsx",
        ".yaml": "yaml",
        ".yml": "yaml",
    }.get(suffix, "")


```
