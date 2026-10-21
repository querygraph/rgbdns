---
type: "code-fragment"
fragment_id: "rgbdns-frag-92fe67fc316b"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_portable_values_pass"
kind: "def"
start_line: 90
end_line: 100
---

# test_portable_values_pass

- Fragment ID: `rgbdns-frag-92fe67fc316b`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 90-100
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-92fe67fc316b", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 90, "endLine": 100}
```

## Excerpt

<span id="rgbdns-frag-92fe67fc316b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-92fe67fc316b: def test_portable_values_pass

```python
    def test_portable_values_pass(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            data = root / checker.BOOK_ROOT / "_data"
            data.mkdir(parents=True)
            (data / "manifest.json").write_text(
                json.dumps({"rgbdns_root": ".", "vault_name": "RGBDNS Book Vault"}), encoding="utf-8"
            )
            self.assertEqual(checker.check_local_paths(root), [])


```
