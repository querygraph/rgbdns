---
type: "code-fragment"
fragment_id: "rgbdns-frag-2af91edb62e1"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_no_duplicate_reader"
kind: "def"
start_line: 44
end_line: 50
---

# test_no_duplicate_reader

- Fragment ID: `rgbdns-frag-2af91edb62e1`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 44-50
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-2af91edb62e1", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 44, "endLine": 50}
```

## Excerpt

<span id="rgbdns-frag-2af91edb62e1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2af91edb62e1: def test_no_duplicate_reader

```python
    def test_no_duplicate_reader(self) -> None:
        self.assertEqual(
            builder.migrate_plugin_list(["firstpair-reader", "rgbdns-reader"]),
            ["firstpair-reader"],
        )


```
