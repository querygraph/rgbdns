# DNS from First Principles — Obsidian Vault

This vault is the reader edition of *DNS from First Principles* by Alexy Khrabrov.
It packages the complete book together with the complete text/code surface of rgbdns.

## Contents

- Book chapters: 31
- rgbdns code-file notes: 96
- Generated code fragments: 696
- rgbdns source commit: `79502939`
- Book source commit: `79502939`

## Start Here

- Open `Home.md` in Obsidian.
- Follow `DNS from First Principles/Book.md` for the chapter map.
- Use `DNS from First Principles/Indices/Code Files.md` for source navigation.
- Use `DNS from First Principles/Indices/Fragments.md` for symbol excerpts.

## Fragment Navigation

The vault includes a local Obsidian plugin named `rgbdns-reader`.
When enabled, generated `rgbdns-fragment` cards can open the related code-file note
and highlight the selected fragment. The plugin is bundled inside `.obsidian/plugins/`
and does not require a build step.

## Data Ledgers

Machine-readable ledgers live under `DNS from First Principles/_data/`:

- `manifest.json` records the build inputs and counts.
- `files.json` records included rgbdns source files.
- `fragments.json` records fragment IDs and line ranges.
- `symbols.json` records extracted Rust and Markdown symbols.
- `links.json` records generated graph edges.
- `units.jsonl` is the publishing compatibility ledger used by FirstPair.

The vault is generated from source. Rebuild it from the book repository rather than
hand-editing generated notes.
