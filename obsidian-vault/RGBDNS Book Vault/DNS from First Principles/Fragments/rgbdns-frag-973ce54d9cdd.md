---
type: "code-fragment"
fragment_id: "rgbdns-frag-973ce54d9cdd"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_detects_local_paths_in_ledgers"
kind: "def"
start_line: 76
end_line: 89
---

# test_detects_local_paths_in_ledgers

- Fragment ID: `rgbdns-frag-973ce54d9cdd`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 76-89
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-973ce54d9cdd", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 76, "endLine": 89}
```

## Excerpt

<span id="rgbdns-frag-973ce54d9cdd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-973ce54d9cdd: def test_detects_local_paths_in_ledgers

```python
    def test_detects_local_paths_in_ledgers(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            data = root / checker.BOOK_ROOT / "_data"
            data.mkdir(parents=True)
            (data / "manifest.json").write_text(
                json.dumps({"rgbdns_root": "/data/data/com.termux/files/home/src/rgbdns"}),
                encoding="utf-8",
            )
            (data / "files.json").write_text(json.dumps([{"summary": "~~~ fences"}]), encoding="utf-8")
            (root / "FIRSTPAIR-VAULT-MANIFEST.json").write_text('{"root": "~/src/x"}', encoding="utf-8")
            errors = checker.check_local_paths(root)
            self.assertEqual(len(errors), 2, errors)

```
