---
type: "code-fragment"
fragment_id: "rgbdns-frag-59677aff8124"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "copy_book_assets"
kind: "def"
start_line: 647
end_line: 653
---

# copy_book_assets

- Fragment ID: `rgbdns-frag-59677aff8124`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 647-653
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-59677aff8124", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 647, "endLine": 653}
```

## Excerpt

<span id="rgbdns-frag-59677aff8124" class="rgbdns-fragment-target"></span>
### rgbdns-frag-59677aff8124: def copy_book_assets

```python
def copy_book_assets(output: Path) -> None:
    assets = output / "Assets"
    assets.mkdir(parents=True, exist_ok=True)
    for name in ("rgbdns-cover.png", "rgbdns-headboard.png"):
        shutil.copy2(REPO_ROOT / "cover" / name, assets / name)


```
