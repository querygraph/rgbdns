---
type: "code-fragment"
fragment_id: "rgbdns-frag-a2353de86c18"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_vault_readme"
kind: "def"
start_line: 514
end_line: 560
---

# render_vault_readme

- Fragment ID: `rgbdns-frag-a2353de86c18`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 514-560
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-a2353de86c18", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 514, "endLine": 560}
```

## Excerpt

<span id="rgbdns-frag-a2353de86c18" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a2353de86c18: def render_vault_readme

```python
def render_vault_readme(manifest: dict[str, object]) -> str:
    return "\n".join([
        "# DNS from First Principles — Obsidian Vault",
        "",
        "This vault is the reader edition of *DNS from First Principles* by Alexy Khrabrov.",
        "It packages the complete book together with the complete text/code surface of rgbdns.",
        "",
        "## Contents",
        "",
        f"- Book chapters: {manifest['chapter_count']}",
        f"- rgbdns code-file notes: {manifest['code_file_count']}",
        f"- Generated code fragments: {manifest['fragment_count']}",
        f"- rgbdns source commit: `{manifest['rgbdns_commit']}`",
        f"- Book source commit: `{manifest['book_commit']}`",
        "",
        "## Start Here",
        "",
        "- Open `Home.md` in Obsidian.",
        "- Follow `DNS from First Principles/Book.md` for the chapter map.",
        "- Use `DNS from First Principles/Indices/Code Files.md` for source navigation.",
        "- Use `DNS from First Principles/Indices/Fragments.md` for symbol excerpts.",
        "",
        "## Fragment Navigation",
        "",
        "The vault bundles FirstPair Reader (`firstpair-reader`) in `.obsidian/plugins/`.",
        "Enable it under Settings -> Community plugins; it needs no build step and makes",
        "no network requests. It opens the chapters with a rail (Previous, Up, Back, Top,",
        "TOC, Next) and turns each generated `rgbdns-fragment` block into a card:",
        "Show code lists the fragment's numbered lines, and Open file opens the code note",
        "with those lines marked. Without the plugin the notes remain a complete book.",
        "",
        "## Data Ledgers",
        "",
        "Machine-readable ledgers live under `DNS from First Principles/_data/`:",
        "",
        "- `manifest.json` records the build inputs and counts.",
        "- `files.json` records included rgbdns source files.",
        "- `fragments.json` records fragment IDs and line ranges.",
        "- `symbols.json` records extracted Rust and Markdown symbols.",
        "- `links.json` records generated graph edges.",
        "- `units.jsonl` is the publishing compatibility ledger used by FirstPair.",
        "",
        "The vault is generated from source. Rebuild it from the book repository rather than",
        "hand-editing generated notes.",
    ])


```
