---
type: "code-fragment"
fragment_id: "rgbdns-frag-859c6d21dd44"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "run"
kind: "def"
start_line: 72
end_line: 152
---

# run

- Fragment ID: `rgbdns-frag-859c6d21dd44`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 72-152
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-859c6d21dd44", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "heading": "rgbdns-frag-859c6d21dd44: def run", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 72, "endLine": 152}
```

## Excerpt

<span id="rgbdns-frag-859c6d21dd44" class="rgbdns-fragment-target"></span>
### rgbdns-frag-859c6d21dd44: def run

```python
def run(root: Path) -> list[str]:
    errors: list[str] = []
    files = {path.relative_to(root).as_posix() for path in root.rglob("*") if path.is_file()}
    notes = {note_key(path, root) for path in root.rglob("*.md")}

    required = [
        "README",
        "Home",
        f"{BOOK_ROOT}/Book",
        f"{BOOK_ROOT}/Indices/Code Files",
        f"{BOOK_ROOT}/Indices/Fragments",
        f"{BOOK_ROOT}/_data/manifest.json",
        f"{BOOK_ROOT}/_data/files.json",
        f"{BOOK_ROOT}/_data/fragments.json",
        f"{BOOK_ROOT}/_data/symbols.json",
        f"{BOOK_ROOT}/_data/links.json",
        f"{BOOK_ROOT}/_data/units.jsonl",
        ".obsidian/plugins/rgbdns-reader/main.js",
        ".obsidian/plugins/rgbdns-reader/manifest.json",
        ".obsidian/plugins/rgbdns-reader/styles.css",
        "Assets/rgbdns-cover.png",
        "Assets/rgbdns-headboard.png",
    ]
    for relative in required:
        if (
            relative.endswith(".json")
            or relative.endswith(".jsonl")
            or relative.endswith(".js")
            or relative.endswith(".css")
            or relative.endswith(".png")
        ):
            if relative not in files:
                errors.append(f"missing required file: {relative}")
        elif relative not in notes:
            errors.append(f"missing required note: {relative}.md")

    data_root = root / BOOK_ROOT / "_data"
    try:
        manifest = load_json(data_root / "manifest.json")
        code_files = load_json(data_root / "files.json")
        fragments = load_json(data_root / "fragments.json")
        symbols = load_json(data_root / "symbols.json")
        links = load_json(data_root / "links.json")
        units = [
            json.loads(line)
            for line in (data_root / "units.jsonl").read_text(encoding="utf-8").splitlines()
            if line.strip()
        ]
    except Exception as exc:
        errors.append(f"could not read data ledgers: {exc}")
        return errors

    if manifest.get("chapter_count", 0) < 28:
        errors.append("manifest reports fewer than 28 chapters")
    if manifest.get("code_file_count", 0) < 60:
        errors.append("manifest reports fewer than 60 code files")
    if manifest.get("fragment_count", 0) < 100:
        errors.append("manifest reports fewer than 100 fragments")
    if len(code_files) != manifest.get("code_file_count"):
        errors.append("files.json count does not match manifest")
    if len(fragments) != manifest.get("fragment_count"):
        errors.append("fragments.json count does not match manifest")
    if len(symbols) < len(fragments):
        errors.append("symbols.json should have at least one row per fragment")
    if len(links) < len(fragments):
        errors.append("links.json should have at least one edge per fragment")
    expected_units = manifest.get("chapter_count", 0) + len(code_files) + len(fragments)
    if len(units) != expected_units:
        errors.append(f"units.jsonl count {len(units)} does not match expected {expected_units}")

    fragment_ids = set()
    for fragment in fragments:
        fid = fragment.get("id")
        if not fid:
            errors.append("fragment row has no id")
            continue
        if fid in fragment_ids:
            errors.append(f"duplicate fragment id: {fid}")
        fragment_ids.add(fid)
        for key in ("note_path", "code_note"):
            value = fragment.get(key)
```
