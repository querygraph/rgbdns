---
type: "code-fragment"
fragment_id: "rgbdns-frag-3b94e6554e61"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "main"
kind: "def"
start_line: 891
end_line: 902
---

# main

- Fragment ID: `rgbdns-frag-3b94e6554e61`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 891-902
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-3b94e6554e61", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 891, "endLine": 902}
```

## Excerpt

<span id="rgbdns-frag-3b94e6554e61" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3b94e6554e61: def main

```python
def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rgbdns-root", type=Path, default=DEFAULT_RGBDNS_ROOT)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()
    manifest = build_vault(args.rgbdns_root.resolve(), args.output.resolve())
    print(json.dumps(manifest, indent=2, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```
