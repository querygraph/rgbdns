---
type: "code-fragment"
fragment_id: "rgbdns-frag-d18c30b0a6e1"
source_path: "FIRSTPAIR.md"
code_note: "DNS from First Principles/Code/FIRSTPAIR.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Obsidian vault"
kind: "heading"
start_line: 28
end_line: 45
---

# Obsidian vault

- Fragment ID: `rgbdns-frag-d18c30b0a6e1`
- Source file: [[DNS from First Principles/Code/FIRSTPAIR.md.source|FIRSTPAIR.md]]
- Lines: 28-45
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-d18c30b0a6e1", "codeNote": "DNS from First Principles/Code/FIRSTPAIR.md.source", "sourcePath": "FIRSTPAIR.md", "startLine": 28, "endLine": 45}
```

## Excerpt

<span id="rgbdns-frag-d18c30b0a6e1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d18c30b0a6e1: heading Obsidian vault

```markdown
## Obsidian vault

Before running `scripts/build-obsidian-vault.py`, editing files under
`obsidian-vault/`, zipping the vault, or otherwise touching it programmatically,
ask the user to close the vault in Obsidian and wait for confirmation. After
regeneration, run `scripts/check-obsidian-vault.py` before delivery.

The vault bundles FirstPair's standard reader, FirstPair Reader 2
(`firstpair-reader`), copied as its four release files from
`$FIRSTPAIR_ROOT/publishing/vault/plugin/firstpair-reader/`. It reads the book
as a code book (`DNS from First Principles/_data/fragments.json` beside
`Chapters/`). The retired `rgbdns-reader` source stays under `obsidian-plugin/`
for reference only. See `docs/OBSIDIAN-VAULT.md`.

`vault.build.json` drives the same builder and validator through FirstPair's
transactional candidate, complete-guide, privacy, comparison, and deterministic
archive gates. The candidate is generated beside the tracked vault and never
replaces it during migration QA.
```
