---
type: "code-fragment"
fragment_id: "rgbdns-frag-3c2e54f9135e"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "load"
kind: "def"
start_line: 19
end_line: 30
---

# load

- Fragment ID: `rgbdns-frag-3c2e54f9135e`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 19-30
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-3c2e54f9135e", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 19, "endLine": 30}
```

## Excerpt

<span id="rgbdns-frag-3c2e54f9135e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3c2e54f9135e: def load

```python
def load(name: str, filename: str):
    spec = importlib.util.spec_from_file_location(name, SCRIPTS / filename)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


builder = load("rgbdns_vault_builder", "build-obsidian-vault.py")
checker = load("rgbdns_vault_checker", "check-obsidian-vault.py")


```
