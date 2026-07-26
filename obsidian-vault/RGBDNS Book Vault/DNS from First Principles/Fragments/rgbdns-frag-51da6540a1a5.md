---
type: "code-fragment"
fragment_id: "rgbdns-frag-51da6540a1a5"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "copy_plugin"
kind: "def"
start_line: 534
end_line: 542
---

# copy_plugin

- Fragment ID: `rgbdns-frag-51da6540a1a5`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 534-542
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-51da6540a1a5", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-51da6540a1a5: def copy_plugin", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 534, "endLine": 542}
```

## Excerpt

<span id="rgbdns-frag-51da6540a1a5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-51da6540a1a5: def copy_plugin

```python
def copy_plugin(output: Path) -> None:
    source = REPO_ROOT / "obsidian-plugin" / "rgbdns-reader"
    target = output / ".obsidian" / "plugins" / "rgbdns-reader"
    if target.exists():
        shutil.rmtree(target)
    shutil.copytree(source, target)
    write_text(output / ".obsidian" / "community-plugins.json", json.dumps(["rgbdns-reader"], indent=2))


```
