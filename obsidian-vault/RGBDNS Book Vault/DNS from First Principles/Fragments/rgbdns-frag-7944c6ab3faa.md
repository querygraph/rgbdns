---
type: "code-fragment"
fragment_id: "rgbdns-frag-7944c6ab3faa"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "local_path_files"
kind: "def"
start_line: 92
end_line: 105
---

# local_path_files

- Fragment ID: `rgbdns-frag-7944c6ab3faa`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 92-105
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-7944c6ab3faa", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 92, "endLine": 105}
```

## Excerpt

<span id="rgbdns-frag-7944c6ab3faa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7944c6ab3faa: def local_path_files

```python
def local_path_files(root: Path) -> list[Path]:
    """Ledgers and manifests that must carry no machine-local paths."""
    candidates: set[Path] = set()
    data_root = root / BOOK_ROOT / "_data"
    if data_root.is_dir():
        candidates.update(path for path in data_root.rglob("*") if path.is_file())
    candidates.update(root.glob("*.json"))
    obsidian = root / ".obsidian"
    if obsidian.is_dir():
        candidates.update(obsidian.glob("*.json"))
        candidates.update(obsidian.glob("plugins/*/manifest.json"))
    return sorted(candidates)


```
