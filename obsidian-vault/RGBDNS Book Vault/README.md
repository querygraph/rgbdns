# DNS from First Principles — Obsidian Vault

This vault is the reader edition of *DNS from First Principles* by Alexy Khrabrov.
It packages the complete book together with the complete text/code surface of rgbdns.

## Contents

- Book chapters: 32
- rgbdns code-file notes: 139
- Generated code fragments: 1097
- rgbdns source commit: `5ae24772`
- Book source commit: `5ae24772`

## Start Here

- Open `Home.md` in Obsidian.
- Follow `DNS from First Principles/Book.md` for the chapter map.
- Use `DNS from First Principles/Indices/Code Files.md` for source navigation.
- Use `DNS from First Principles/Indices/Fragments.md` for symbol excerpts.

## Fragment Navigation

The vault bundles FirstPair Reader (`firstpair-reader`) in `.obsidian/plugins/`.
Enable it under Settings -> Community plugins; it needs no build step and makes
no network requests. It opens the chapters with a rail (Previous, Up, Back, Top,
TOC, Next) and turns each generated `rgbdns-fragment` block into a card:
Show code lists the fragment's numbered lines, and Open file opens the code note
with those lines marked. Without the plugin the notes remain a complete book.

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
