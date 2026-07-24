---
type: "code-file"
source_path: "docs/OBSIDIAN-VAULT.md"
language: "markdown"
subsystem: "Documentation"
line_count: 34
fragment_count: 3
rgbdns_commit: "472c2087"
---

# docs/OBSIDIAN-VAULT.md

- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]
- Source path: `docs/OBSIDIAN-VAULT.md`
- Lines: 34
- Summary: DNS from First Principles Obsidian vault

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-1c7366ed3996|DNS from First Principles Obsidian vault]]: lines 1-7
- [[DNS from First Principles/Fragments/rgbdns-frag-b9c796f4839e|Rebuild]]: lines 8-21
- [[DNS from First Principles/Fragments/rgbdns-frag-02be59d90865|Reader map]]: lines 22-34

## Full Source

```markdown
# DNS from First Principles Obsidian vault

The committed vault is a generated reader edition of *DNS from First
Principles*. It contains the complete manuscript, the repository’s complete
text/code surface, symbol-level excerpts, subsystem and component maps, and the
bundled `rgbdns-reader` plugin.

## Rebuild

Close the vault in Obsidian before rebuilding it, then run:

```sh
python3 scripts/build-obsidian-vault.py
python3 scripts/check-obsidian-vault.py \
  "obsidian-vault/RGBDNS Book Vault"
```

The generator reads `docs/book/rgbdns.md` and the current repository checkout.
Set `RGBDNS_CODE_ROOT` or pass `--rgbdns-root` only when deliberately building
against another rgbdns checkout.

## Reader map

Open `Home.md`, then `DNS from First Principles/Book.md`. The book map links
the canonical chapters, code files, extracted symbols, components, and
subsystems. Generated fragment cards open the corresponding collocated code
note and highlight the selected excerpt.

The reader plugin also provides commands to open the book map and move to the
next or previous chapter. It shows chapter progress in Obsidian’s status bar
and applies a focused reading width to chapter notes.

The vault is generated. Change the manuscript, source code, generator, or
plugin sources and rebuild instead of hand-editing generated notes.
```
