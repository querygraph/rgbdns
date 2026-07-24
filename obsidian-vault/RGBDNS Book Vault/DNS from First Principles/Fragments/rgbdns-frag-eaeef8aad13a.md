---
type: "code-fragment"
fragment_id: "rgbdns-frag-eaeef8aad13a"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "copy_plugin"
kind: "def"
start_line: 518
end_line: 526
---

# copy_plugin

- Fragment ID: `rgbdns-frag-eaeef8aad13a`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 518-526
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-eaeef8aad13a", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-eaeef8aad13a: def copy_plugin", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 518, "endLine": 526}
```

## Excerpt

<span id="rgbdns-frag-eaeef8aad13a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-eaeef8aad13a: def copy_plugin

```python
def copy_plugin(output: Path) -> None:
    source = REPO_ROOT / "obsidian-plugin" / "rgbdns-reader"
    target = output / ".obsidian" / "plugins" / "rgbdns-reader"
    if target.exists():
        shutil.rmtree(target)
    shutil.copytree(source, target)
    write_text(output / ".obsidian" / "community-plugins.json", json.dumps(["rgbdns-reader"], indent=2))


```
