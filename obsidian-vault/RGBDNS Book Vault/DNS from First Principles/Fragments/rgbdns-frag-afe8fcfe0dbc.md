---
type: "code-fragment"
fragment_id: "rgbdns-frag-afe8fcfe0dbc"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_repo_root_is_dot"
kind: "def"
start_line: 52
end_line: 54
---

# test_repo_root_is_dot

- Fragment ID: `rgbdns-frag-afe8fcfe0dbc`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 52-54
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-afe8fcfe0dbc", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 52, "endLine": 54}
```

## Excerpt

<span id="rgbdns-frag-afe8fcfe0dbc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-afe8fcfe0dbc: def test_repo_root_is_dot

```python
    def test_repo_root_is_dot(self) -> None:
        self.assertEqual(builder.portable_root(builder.REPO_ROOT), ".")

```
