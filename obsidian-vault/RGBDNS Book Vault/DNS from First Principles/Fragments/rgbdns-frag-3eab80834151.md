---
type: "code-fragment"
fragment_id: "rgbdns-frag-3eab80834151"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "main"
kind: "def"
start_line: 185
end_line: 208
---

# main

- Fragment ID: `rgbdns-frag-3eab80834151`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 185-208
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-3eab80834151", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "heading": "rgbdns-frag-3eab80834151: def main", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 185, "endLine": 208}
```

## Excerpt

<span id="rgbdns-frag-3eab80834151" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3eab80834151: def main

```python
def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("vault", type=Path)
    args = parser.parse_args()
    root = args.vault.resolve()
    errors = run(root)
    if errors:
        for error in errors[:200]:
            print(error, file=sys.stderr)
        if len(errors) > 200:
            print(f"... {len(errors) - 200} more errors", file=sys.stderr)
        return 1
    manifest = load_json(root / BOOK_ROOT / "_data" / "manifest.json")
    print(
        "vault ok: "
        f"{manifest['chapter_count']} chapters, "
        f"{manifest['code_file_count']} code files, "
        f"{manifest['fragment_count']} fragments"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```
