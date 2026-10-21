---
type: "code-fragment"
fragment_id: "rgbdns-frag-696eeed7312b"
source_path: "docs/OBSIDIAN-VAULT.md"
code_note: "DNS from First Principles/Code/docs/OBSIDIAN-VAULT.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Rebuild"
kind: "heading"
start_line: 8
end_line: 43
---

# Rebuild

- Fragment ID: `rgbdns-frag-696eeed7312b`
- Source file: [[DNS from First Principles/Code/docs/OBSIDIAN-VAULT.md.source|docs/OBSIDIAN-VAULT.md]]
- Lines: 8-43
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-696eeed7312b", "codeNote": "DNS from First Principles/Code/docs/OBSIDIAN-VAULT.md.source", "sourcePath": "docs/OBSIDIAN-VAULT.md", "startLine": 8, "endLine": 43}
```

## Excerpt

<span id="rgbdns-frag-696eeed7312b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-696eeed7312b: heading Rebuild

````markdown
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

````
