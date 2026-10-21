---
type: "code-fragment"
fragment_id: "rgbdns-frag-205b4c6a0f96"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "make"
kind: "def"
start_line: 102
end_line: 109
---

# make

- Fragment ID: `rgbdns-frag-205b4c6a0f96`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 102-109
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-205b4c6a0f96", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 102, "endLine": 109}
```

## Excerpt

<span id="rgbdns-frag-205b4c6a0f96" class="rgbdns-fragment-target"></span>
### rgbdns-frag-205b4c6a0f96: def make

```python
    def make(self, root: Path, plugins: list[str], files: tuple[str, ...]) -> None:
        plugin = root / ".obsidian" / "plugins" / "firstpair-reader"
        plugin.mkdir(parents=True)
        for name in files:
            content = json.dumps({"id": "firstpair-reader"}) if name == "manifest.json" else ""
            (plugin / name).write_text(content, encoding="utf-8")
        (root / ".obsidian" / "community-plugins.json").write_text(json.dumps(plugins), encoding="utf-8")

```
