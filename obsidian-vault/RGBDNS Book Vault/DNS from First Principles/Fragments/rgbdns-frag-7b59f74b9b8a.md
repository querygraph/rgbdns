---
type: "code-fragment"
fragment_id: "rgbdns-frag-7b59f74b9b8a"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_external_root_is_symbolic"
kind: "def"
start_line: 55
end_line: 59
---

# test_external_root_is_symbolic

- Fragment ID: `rgbdns-frag-7b59f74b9b8a`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 55-59
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-7b59f74b9b8a", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 55, "endLine": 59}
```

## Excerpt

<span id="rgbdns-frag-7b59f74b9b8a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7b59f74b9b8a: def test_external_root_is_symbolic

```python
    def test_external_root_is_symbolic(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            self.assertEqual(builder.portable_root(Path(temporary)), "$RGBDNS_CODE_ROOT")


```
