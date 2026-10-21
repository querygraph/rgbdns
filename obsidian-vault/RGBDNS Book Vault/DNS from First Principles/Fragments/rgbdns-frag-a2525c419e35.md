---
type: "code-fragment"
fragment_id: "rgbdns-frag-a2525c419e35"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "check_code_book_contract"
kind: "def"
start_line: 162
end_line: 183
---

# check_code_book_contract

- Fragment ID: `rgbdns-frag-a2525c419e35`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 162-183
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-a2525c419e35", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 162, "endLine": 183}
```

## Excerpt

<span id="rgbdns-frag-a2525c419e35" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a2525c419e35: def check_code_book_contract

```python
def check_code_book_contract(root: Path) -> list[str]:
    """FirstPair Reader 2 code books: <Book>/_data/fragments.json beside <Book>/Chapters/."""
    errors: list[str] = []
    book = root / BOOK_ROOT
    if not (book / "_data" / "fragments.json").is_file():
        errors.append(f"code book has no {BOOK_ROOT}/_data/fragments.json")
    chapters = sorted((book / "Chapters").glob("*.md")) if (book / "Chapters").is_dir() else []
    if not chapters:
        errors.append(f"code book has no chapter notes in {BOOK_ROOT}/Chapters/")
    if not (book / "Book.md").is_file():
        errors.append(f"code book has no launcher {BOOK_ROOT}/Book.md")
    # Reader 2 discovers the vault root first, then top-level folders in name
    # order; nothing may claim the Reader before the book folder.
    for shadow in ("_data", "Reader/reader-index.json"):
        if (root / shadow).exists():
            errors.append(f"vault root holds {shadow}, which FirstPair Reader would read instead of the book")
    for folder in sorted(path for path in root.iterdir() if path.is_dir() and not path.name.startswith(".")):
        if folder.name != BOOK_ROOT and (folder / "_data").exists():
            errors.append(f"{folder.name}/_data competes with the book for FirstPair Reader discovery")
    return errors


```
