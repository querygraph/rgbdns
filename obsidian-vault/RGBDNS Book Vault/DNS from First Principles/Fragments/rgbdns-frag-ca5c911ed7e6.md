---
type: "code-fragment"
fragment_id: "rgbdns-frag-ca5c911ed7e6"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_legacy_and_sources_fail"
kind: "def"
start_line: 116
end_line: 124
---

# test_legacy_and_sources_fail

- Fragment ID: `rgbdns-frag-ca5c911ed7e6`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 116-124
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-ca5c911ed7e6", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 116, "endLine": 124}
```

## Excerpt

<span id="rgbdns-frag-ca5c911ed7e6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ca5c911ed7e6: def test_legacy_and_sources_fail

```python
    def test_legacy_and_sources_fail(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.make(root, ["rgbdns-reader"], (*checker.READER_RELEASE_FILES, "package.json"))
            (root / ".obsidian" / "plugins" / "rgbdns-reader").mkdir()
            errors = checker.check_reader_plugin(root)
            self.assertEqual(len(errors), 3, errors)


```
