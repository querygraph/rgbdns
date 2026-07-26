---
type: "code-fragment"
fragment_id: "rgbdns-frag-99bc96942e75"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "copy_book_assets"
kind: "def"
start_line: 543
end_line: 549
---

# copy_book_assets

- Fragment ID: `rgbdns-frag-99bc96942e75`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 543-549
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-99bc96942e75", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-99bc96942e75: def copy_book_assets", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 543, "endLine": 549}
```

## Excerpt

<span id="rgbdns-frag-99bc96942e75" class="rgbdns-fragment-target"></span>
### rgbdns-frag-99bc96942e75: def copy_book_assets

```python
def copy_book_assets(output: Path) -> None:
    assets = output / "Assets"
    assets.mkdir(parents=True, exist_ok=True)
    for name in ("rgbdns-cover.png", "rgbdns-headboard.png"):
        shutil.copy2(REPO_ROOT / "cover" / name, assets / name)


```
