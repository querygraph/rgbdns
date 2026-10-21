---
type: "code-fragment"
fragment_id: "rgbdns-frag-5baf88436b54"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_release_install_passes"
kind: "def"
start_line: 110
end_line: 115
---

# test_release_install_passes

- Fragment ID: `rgbdns-frag-5baf88436b54`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 110-115
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-5baf88436b54", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 110, "endLine": 115}
```

## Excerpt

<span id="rgbdns-frag-5baf88436b54" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5baf88436b54: def test_release_install_passes

```python
    def test_release_install_passes(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.make(root, ["firstpair-reader"], checker.READER_RELEASE_FILES)
            self.assertEqual(checker.check_reader_plugin(root), [])

```
