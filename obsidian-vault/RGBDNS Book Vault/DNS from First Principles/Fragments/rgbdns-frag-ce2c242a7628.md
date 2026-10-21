---
type: "code-fragment"
fragment_id: "rgbdns-frag-ce2c242a7628"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_legacy_id_replaced_in_place"
kind: "def"
start_line: 35
end_line: 40
---

# test_legacy_id_replaced_in_place

- Fragment ID: `rgbdns-frag-ce2c242a7628`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 35-40
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-ce2c242a7628", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 35, "endLine": 40}
```

## Excerpt

<span id="rgbdns-frag-ce2c242a7628" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ce2c242a7628: def test_legacy_id_replaced_in_place

```python
    def test_legacy_id_replaced_in_place(self) -> None:
        self.assertEqual(
            builder.migrate_plugin_list(["dataview", "rgbdns-reader", "outliner"]),
            ["dataview", "firstpair-reader", "outliner"],
        )

```
