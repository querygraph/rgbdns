---
type: "code-fragment"
fragment_id: "rgbdns-frag-f3fcdf78c7fa"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "language_for"
kind: "def"
start_line: 183
end_line: 204
---

# language_for

- Fragment ID: `rgbdns-frag-f3fcdf78c7fa`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 183-204
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-f3fcdf78c7fa", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 183, "endLine": 204}
```

## Excerpt

<span id="rgbdns-frag-f3fcdf78c7fa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f3fcdf78c7fa: def language_for

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
