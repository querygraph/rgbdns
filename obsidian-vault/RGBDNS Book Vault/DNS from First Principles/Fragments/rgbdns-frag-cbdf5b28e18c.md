---
type: "code-fragment"
fragment_id: "rgbdns-frag-cbdf5b28e18c"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "run_git"
kind: "def"
start_line: 119
end_line: 125
---

# run_git

- Fragment ID: `rgbdns-frag-cbdf5b28e18c`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 119-125
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-cbdf5b28e18c", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 119, "endLine": 125}
```

## Excerpt

<span id="rgbdns-frag-cbdf5b28e18c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cbdf5b28e18c: def run_git

```python
def run_git(args: list[str], cwd: Path) -> str:
    try:
        return subprocess.check_output(["git", *args], cwd=cwd, text=True).strip()
    except Exception:
        return "unknown"


```
