---
type: "code-fragment"
fragment_id: "rgbdns-frag-52a4762a4e84"
source_path: "AGENTS.md"
code_note: "DNS from First Principles/Code/AGENTS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Obsidian vault"
kind: "heading"
start_line: 126
end_line: 141
---

# Obsidian vault

- Fragment ID: `rgbdns-frag-52a4762a4e84`
- Source file: [[DNS from First Principles/Code/AGENTS.md.source|AGENTS.md]]
- Lines: 126-141
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-52a4762a4e84", "codeNote": "DNS from First Principles/Code/AGENTS.md.source", "sourcePath": "AGENTS.md", "startLine": 126, "endLine": 141}
```

## Excerpt

<span id="rgbdns-frag-52a4762a4e84" class="rgbdns-fragment-target"></span>
### rgbdns-frag-52a4762a4e84: heading Obsidian vault

```markdown
## Obsidian vault

- The vault uses FirstPair Reader 2 (`firstpair-reader`) from
  `$FIRSTPAIR_ROOT` (default `~/src/firstpair`); the builder installs only its
  release files and fails if they are missing or `main.js` is stale. Do not
  reinstall `obsidian-plugin/rgbdns-reader`, which is retired.
- Clean outputs seed an empty `community-plugins.json`; rebuilds over an
  existing vault replace `rgbdns-reader` with `firstpair-reader` in place.
- Never record absolute local paths in vault ledgers or manifests; the
  validator rejects them.
- Close the vault in Obsidian before building (see `FIRSTPAIR.md`); build
  migration candidates into a new sibling folder under `obsidian-vault/`, never
  over the tracked vault. Validate with `scripts/check-obsidian-vault.py` and
  `python3 -m unittest scripts/test_obsidian_vault.py`. Details:
  `docs/OBSIDIAN-VAULT.md`.

```
