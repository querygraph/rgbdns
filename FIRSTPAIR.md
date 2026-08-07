# FirstPair Library Contract

slug: rgbdns
shelf: technology
default_edition: full

This repository owns the manuscript, build declaration, source-specific build
fallback, version metadata, and canonical artifacts for *DNS from First
Principles*. The central FirstPair repository, when available at
`~/src/firstpair`, owns catalog assembly, publishing, hosted readers, and
deployment.

## Build

From the repository root:

```sh
docs/book/build.sh
```

The wrapper uses the central FirstPair builder when it is installed. Otherwise
it performs the same source-owned core workflow with Pandoc and Typst and emits
PDF, EPUB, standalone HTML, and `VERSION.md` under `docs/book/dist/`.

Building does not publish. Public catalog or deployment actions require an
explicit publishing request and the central FirstPair publishing workflow.

## Obsidian vault

Before running `scripts/build-obsidian-vault.py`, editing files under
`obsidian-vault/`, zipping the vault, or otherwise touching it programmatically,
ask the user to close the vault in Obsidian and wait for confirmation. After
regeneration, run `scripts/check-obsidian-vault.py` before delivery.

`vault.build.json` drives the same builder and validator through FirstPair's
transactional candidate, complete-guide, privacy, comparison, and deterministic
archive gates. The candidate is generated beside the tracked vault and never
replaces it during migration QA.
