---
type: "code-fragment"
fragment_id: "rgbdns-frag-e6ed83fc8e4a"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "build_vault"
kind: "def"
start_line: 654
end_line: 734
---

# build_vault

- Fragment ID: `rgbdns-frag-e6ed83fc8e4a`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 654-734
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-e6ed83fc8e4a", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 654, "endLine": 734}
```

## Excerpt

<span id="rgbdns-frag-e6ed83fc8e4a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e6ed83fc8e4a: def build_vault

```python
def build_vault(rgbdns_root: Path, output: Path) -> dict[str, object]:
    plugin_source = reader_plugin_source()
    previous_plugins = read_plugin_list(output)
    if output.exists():
        shutil.rmtree(output)
    output.mkdir(parents=True)

    rgbdns_commit = run_git(["rev-parse", "--short=8", "HEAD"], rgbdns_root)
    book_commit = run_git(["rev-parse", "--short=8", "HEAD"], REPO_ROOT)
    sources = inventory_codebase(rgbdns_root)
    fragments = [fragment for source in sources for fragment in source.fragments]

    write_note(output, "Home", {
        "type": "vault",
        "book": "DNS from First Principles",
        "rgbdns_commit": rgbdns_commit,
        "book_commit": book_commit,
    }, "\n".join([
        "# DNS from First Principles",
        "",
        "![[Assets/rgbdns-headboard.png]]",
        "",
        f"Open {wiki(f'{VAULT_BOOK}/Book', 'the book map')} to start.",
        "",
        "This generated Obsidian vault collocates the book text with the current rgbdns codebase.",
        "With FirstPair Reader enabled, fragment cards show the numbered code lines in place",
        "or open the code note with those lines marked.",
    ]))

    chapter_links: list[tuple[str, str]] = []
    for title, note_path, text in manuscript_chapters():
        body = render_chapter(title, text, fragments)
        chapter_links.append((title, note_path))
        write_note(output, note_path, {
            "type": "chapter",
            "source_file": MANUSCRIPT.relative_to(REPO_ROOT).as_posix(),
        }, body)

    write_note(output, f"{VAULT_BOOK}/Book", {
        "type": "book",
        "rgbdns_commit": rgbdns_commit,
        "book_commit": book_commit,
        "chapter_count": len(chapter_links),
        "code_file_count": len(sources),
        "fragment_count": len(fragments),
    }, "\n".join([
        "# DNS from First Principles",
        "",
        "## Chapters",
        "",
        *[f"- {wiki(path, title)}" for title, path in chapter_links],
        "",
        "## Codebase",
        "",
        f"- {wiki(f'{VAULT_BOOK}/Indices/Code Files', 'Code files')}",
        f"- {wiki(f'{VAULT_BOOK}/Indices/Fragments', 'Code fragments')}",
        f"- {wiki(f'{VAULT_BOOK}/Indices/Components', 'Components')}",
        f"- {wiki(f'{VAULT_BOOK}/Indices/Subsystems', 'Subsystems')}",
    ]))

    crate_items: dict[str, list[SourceFile]] = {}
    subsystem_items: dict[str, list[SourceFile]] = {}
    for source in sources:
        if source.crate:
            crate_items.setdefault(source.crate, []).append(source)
        subsystem_items.setdefault(source.subsystem, []).append(source)
        write_note(output, source.note_path, {
            "type": "code-file",
            "source_path": source.path,
            "language": source.language,
            "subsystem": source.subsystem,
            "crate": source.crate,
            "line_count": len(source.lines),
            "fragment_count": len(source.fragments),
            "rgbdns_commit": rgbdns_commit,
        }, render_source_file(source))

    sources_by_path = {source.path: source for source in sources}
    for fragment in fragments:
        source = sources_by_path[fragment.source_path]
        write_note(output, fragment.note_path, {
```
