---
type: "code-fragment"
fragment_id: "rgbdns-frag-d186e032be32"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "check_local_paths"
kind: "def"
start_line: 106
end_line: 122
---

# check_local_paths

- Fragment ID: `rgbdns-frag-d186e032be32`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 106-122
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-d186e032be32", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 106, "endLine": 122}
```

## Excerpt

<span id="rgbdns-frag-d186e032be32" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d186e032be32: def check_local_paths

```python
def check_local_paths(root: Path) -> list[str]:
    errors: list[str] = []
    for path in local_path_files(root):
        rel = path.relative_to(root).as_posix()
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError) as exc:
            errors.append(f"{rel}: unreadable while scanning for local paths: {exc}")
            continue
        for number, line in enumerate(text.splitlines(), start=1):
            match = LOCAL_PATH_RE.search(line)
            if match:
                errors.append(f"{rel}:{number}: machine-local absolute path {match.group(0)!r}")
                break
    return errors


```
