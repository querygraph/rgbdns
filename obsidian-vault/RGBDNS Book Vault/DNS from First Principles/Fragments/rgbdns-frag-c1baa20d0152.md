---
type: "code-fragment"
fragment_id: "rgbdns-frag-c1baa20d0152"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "main"
kind: "def"
start_line: 783
end_line: 794
---

# main

- Fragment ID: `rgbdns-frag-c1baa20d0152`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 783-794
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-c1baa20d0152", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-c1baa20d0152: def main", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 783, "endLine": 794}
```

## Excerpt

<span id="rgbdns-frag-c1baa20d0152" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c1baa20d0152: def main

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
