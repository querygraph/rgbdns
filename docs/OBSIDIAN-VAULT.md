# DNS from First Principles Obsidian vault

The committed vault is a generated reader edition of *DNS from First
Principles*. It contains the complete manuscript, the repository’s complete
text/code surface, symbol-level excerpts, subsystem and component maps, and the
bundled FirstPair Reader 2 plugin (`firstpair-reader`).

## Rebuild

Close the vault in Obsidian before rebuilding it, then run:

```sh
python3 scripts/build-obsidian-vault.py
python3 scripts/check-obsidian-vault.py \
  "obsidian-vault/RGBDNS Book Vault"
python3 -m unittest scripts/test_obsidian_vault.py
```

The generator reads `docs/book/rgbdns.md` and the current repository checkout.
Set `RGBDNS_CODE_ROOT` or pass `--rgbdns-root` only when deliberately building
against another rgbdns checkout.

The reader plugin comes from FirstPair, not from this repository. The builder
copies exactly its four release files (`README.md`, `main.js`,
`manifest.json`, `styles.css`) from
`$FIRSTPAIR_ROOT/publishing/vault/plugin/firstpair-reader/` (default
`~/src/firstpair`) and stops before touching the vault if a file is missing or
`main.js` is stale against its sources
(`bundle-obsidian-plugin.py --check`).

Enablement follows FirstPair's rule. A clean output seeds an empty
`.obsidian/community-plugins.json`, so the reader enables the plugin
deliberately. A rebuild over an existing vault keeps that vault's list and
order, replacing the retired `rgbdns-reader` id with `firstpair-reader` in
place and removing `.obsidian/plugins/rgbdns-reader/`. Other `.obsidian/`
state is not preserved; the vault is regenerated whole.

The ledger `_data/manifest.json` records only portable values: the code root
relative to this repository (`.`, or `$RGBDNS_CODE_ROOT` for an external
checkout), the vault folder name, commits, and counts. The validator fails on
machine-local paths (`/Users/`, `/home/`, `/data/data/`, `~/`) in `_data/`,
top-level JSON manifests, and `.obsidian` JSON and plugin manifests.

## Reader map

Enable **FirstPair Reader** under Settings → Community plugins. Open `Home.md`,
then `DNS from First Principles/Book.md`; the Reader opens the 31 chapters (in
file-name order from `Chapters/`) with its rail: Previous, Up, Back, Top, TOC,
Next. The book map also links the code files, extracted symbols, components,
and subsystems.

Each chapter ends with generated `rgbdns-fragment` cards, and each fragment
note carries its own card. The Reader resolves a card through
`_data/fragments.json`: **Show code** lists the fragment's numbered lines from
the code note's `## Full Source` block, and **Open file** opens the code note in
the Reader with those lines marked. Each code note wraps its source in a fence
longer than any fence inside it, so Markdown sources with their own code
blocks stay whole and verbatim. The validator checks that every fragment's
code note exists and that the Reader would read at least `end_line` lines from
it. Without the plugin the notes remain a complete book.

Known limits, unchanged by the Reader 2 migration: chapter cards are chosen by
a title-keyword mapping and appended at the chapter's end rather than anchored
in the prose, and fragments are capped at 80 lines and at 80 per file.

The vault is generated. Change the manuscript, source code, or generator and
rebuild instead of hand-editing generated notes.
