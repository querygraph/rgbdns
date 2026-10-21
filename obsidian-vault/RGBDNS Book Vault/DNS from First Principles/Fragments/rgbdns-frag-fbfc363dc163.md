---
type: "code-fragment"
fragment_id: "rgbdns-frag-fbfc363dc163"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_missing_full_source"
kind: "def"
start_line: 71
end_line: 74
---

# test_missing_full_source

- Fragment ID: `rgbdns-frag-fbfc363dc163`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 71-74
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-fbfc363dc163", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 71, "endLine": 74}
```

## Excerpt

<span id="rgbdns-frag-fbfc363dc163" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fbfc363dc163: def test_missing_full_source

```python
    def test_missing_full_source(self) -> None:
        self.assertIsNone(checker.reader_excerpt_lines("# note\n\n```rust\nfn x() {}\n```\n"))


```
