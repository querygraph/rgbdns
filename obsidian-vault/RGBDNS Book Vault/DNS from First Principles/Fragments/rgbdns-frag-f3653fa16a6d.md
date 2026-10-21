---
type: "code-fragment"
fragment_id: "rgbdns-frag-f3653fa16a6d"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_firstpair_reader_release_is_current"
kind: "def"
start_line: 126
end_line: 135
---

# test_firstpair_reader_release_is_current

- Fragment ID: `rgbdns-frag-f3653fa16a6d`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 126-135
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-f3653fa16a6d", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 126, "endLine": 135}
```

## Excerpt

<span id="rgbdns-frag-f3653fa16a6d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f3653fa16a6d: def test_firstpair_reader_release_is_current

```python
    def test_firstpair_reader_release_is_current(self) -> None:
        if not (builder.FIRSTPAIR_ROOT / "publishing" / "vault" / "plugin" / "firstpair-reader").is_dir():
            self.skipTest("FirstPair checkout not available")
        source = builder.reader_plugin_source()
        manifest = json.loads((source / "manifest.json").read_text(encoding="utf-8"))
        self.assertEqual(manifest["id"], "firstpair-reader")


if __name__ == "__main__":
    unittest.main()
```
