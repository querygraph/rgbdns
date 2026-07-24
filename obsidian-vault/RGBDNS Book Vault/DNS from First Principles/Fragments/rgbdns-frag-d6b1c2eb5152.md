---
type: "code-fragment"
fragment_id: "rgbdns-frag-d6b1c2eb5152"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "extract_fragments"
kind: "def"
start_line: 272
end_line: 315
---

# extract_fragments

- Fragment ID: `rgbdns-frag-d6b1c2eb5152`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 272-315
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-d6b1c2eb5152", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-d6b1c2eb5152: def extract_fragments", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 272, "endLine": 315}
```

## Excerpt

<span id="rgbdns-frag-d6b1c2eb5152" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d6b1c2eb5152: def extract_fragments

```python
def extract_fragments(source: SourceFile) -> list[Fragment]:
    matches: list[tuple[int, str, str]] = []
    regex = RUST_SYMBOL_RE if source.language == "rust" else PY_SYMBOL_RE if source.language == "python" else None
    if regex:
        for index, line in enumerate(source.lines, start=1):
            match = regex.match(line)
            if not match:
                continue
            kind = match.group("kind").replace(" ", "-")
            name = match.groupdict().get("name") or "impl"
            matches.append((index, kind, name))
    elif source.language == "markdown":
        for index, line in enumerate(source.lines, start=1):
            match = MD_HEADING_RE.match(line)
            if match and len(match.group("marks")) <= 2:
                matches.append((index, "heading", clean_name(match.group("name"), 80)))

    if not matches and source.lines:
        matches.append((1, "file", Path(source.path).name))

    fragments: list[Fragment] = []
    for i, (start, kind, name) in enumerate(matches[:80]):
        next_start = matches[i + 1][0] if i + 1 < len(matches) else len(source.lines) + 1
        end = min(next_start - 1, start + 80, len(source.lines))
        fid = fragment_id(source.path, start, end, f"{kind}:{name}")
        heading = f"{fid}: {kind} {name}"
        fragments.append(Fragment(
            id=fid,
            source_path=source.path,
            note_path=f"{VAULT_BOOK}/Fragments/{fid}",
            code_note=source.note_path,
            heading=heading,
            symbol=name,
            kind=kind,
            language=source.language,
            subsystem=source.subsystem,
            crate=source.crate,
            start_line=start,
            end_line=end,
            summary=f"{kind} `{name}` in `{source.path}` lines {start}-{end}.",
        ))
    return fragments


```
