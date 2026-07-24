---
type: "code-fragment"
fragment_id: "rgbdns-frag-bdd14a990594"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "run_git"
kind: "def"
start_line: 111
end_line: 117
---

# run_git

- Fragment ID: `rgbdns-frag-bdd14a990594`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 111-117
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-bdd14a990594", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-bdd14a990594: def run_git", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 111, "endLine": 117}
```

## Excerpt

<span id="rgbdns-frag-bdd14a990594" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bdd14a990594: def run_git

```python
def run_git(args: list[str], cwd: Path) -> str:
    try:
        return subprocess.check_output(["git", *args], cwd=cwd, text=True).strip()
    except Exception:
        return "unknown"


```
