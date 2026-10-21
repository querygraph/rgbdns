---
type: "code-fragment"
fragment_id: "rgbdns-frag-3f2d366ffc26"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_disabled_reader_stays_disabled"
kind: "def"
start_line: 41
end_line: 43
---

# test_disabled_reader_stays_disabled

- Fragment ID: `rgbdns-frag-3f2d366ffc26`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 41-43
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-3f2d366ffc26", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 41, "endLine": 43}
```

## Excerpt

<span id="rgbdns-frag-3f2d366ffc26" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3f2d366ffc26: def test_disabled_reader_stays_disabled

```python
    def test_disabled_reader_stays_disabled(self) -> None:
        self.assertEqual(builder.migrate_plugin_list(["dataview"]), ["dataview"])

```
