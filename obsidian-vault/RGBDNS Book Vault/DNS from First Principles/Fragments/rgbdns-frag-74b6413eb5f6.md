---
type: "code-fragment"
fragment_id: "rgbdns-frag-74b6413eb5f6"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "inventory_codebase"
kind: "def"
start_line: 316
end_line: 341
---

# inventory_codebase

- Fragment ID: `rgbdns-frag-74b6413eb5f6`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 316-341
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-74b6413eb5f6", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-74b6413eb5f6: def inventory_codebase", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 316, "endLine": 341}
```

## Excerpt

<span id="rgbdns-frag-74b6413eb5f6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-74b6413eb5f6: def inventory_codebase

```python
def inventory_codebase(rgbdns_root: Path) -> list[SourceFile]:
    files: list[SourceFile] = []
    for path in sorted(rgbdns_root.rglob("*")):
        if not should_include(path, rgbdns_root):
            continue
        text = read_text(path)
        if text is None:
            continue
        rel = path.relative_to(rgbdns_root).as_posix()
        subsystem, crate = subsystem_for(rel)
        lines = text.splitlines()
        source = SourceFile(
            path=rel,
            absolute=path,
            note_path=code_note_path(rel),
            language=language_for(rel),
            subsystem=subsystem,
            crate=crate,
            lines=lines,
            summary=summary_for(rel, subsystem, lines),
        )
        source.fragments = extract_fragments(source)
        files.append(source)
    return files


```
