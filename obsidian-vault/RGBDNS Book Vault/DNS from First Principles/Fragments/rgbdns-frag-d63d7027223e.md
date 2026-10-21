---
type: "code-fragment"
fragment_id: "rgbdns-frag-d63d7027223e"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "inventory_codebase"
kind: "def"
start_line: 334
end_line: 359
---

# inventory_codebase

- Fragment ID: `rgbdns-frag-d63d7027223e`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 334-359
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-d63d7027223e", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 334, "endLine": 359}
```

## Excerpt

<span id="rgbdns-frag-d63d7027223e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d63d7027223e: def inventory_codebase

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
