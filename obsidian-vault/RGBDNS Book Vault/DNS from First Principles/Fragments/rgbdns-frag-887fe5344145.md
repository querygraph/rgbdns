---
type: "code-fragment"
fragment_id: "rgbdns-frag-887fe5344145"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "copy_book_assets"
kind: "def"
start_line: 527
end_line: 533
---

# copy_book_assets

- Fragment ID: `rgbdns-frag-887fe5344145`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 527-533
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-887fe5344145", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-887fe5344145: def copy_book_assets", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 527, "endLine": 533}
```

## Excerpt

<span id="rgbdns-frag-887fe5344145" class="rgbdns-fragment-target"></span>
### rgbdns-frag-887fe5344145: def copy_book_assets

```python
def copy_book_assets(output: Path) -> None:
    assets = output / "Assets"
    assets.mkdir(parents=True, exist_ok=True)
    for name in ("rgbdns-cover.png", "rgbdns-headboard.png"):
        shutil.copy2(REPO_ROOT / "cover" / name, assets / name)


```
