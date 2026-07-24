---
type: "code-fragment"
fragment_id: "rgbdns-frag-e91f6f32bf5b"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "render_vault_readme"
kind: "def"
start_line: 473
end_line: 517
---

# render_vault_readme

- Fragment ID: `rgbdns-frag-e91f6f32bf5b`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 473-517
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-e91f6f32bf5b", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "heading": "rgbdns-frag-e91f6f32bf5b: def render_vault_readme", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 473, "endLine": 517}
```

## Excerpt

<span id="rgbdns-frag-e91f6f32bf5b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e91f6f32bf5b: def render_vault_readme

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
        "The vault includes a local Obsidian plugin named `rgbdns-reader`.",
        "When enabled, generated `rgbdns-fragment` cards can open the related code-file note",
        "and highlight the selected fragment. The plugin is bundled inside `.obsidian/plugins/`",
        "and does not require a build step.",
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
