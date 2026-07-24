---
type: "code-fragment"
fragment_id: "rgbdns-frag-b9c796f4839e"
source_path: "docs/OBSIDIAN-VAULT.md"
code_note: "DNS from First Principles/Code/docs/OBSIDIAN-VAULT.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Rebuild"
kind: "heading"
start_line: 8
end_line: 21
---

# Rebuild

- Fragment ID: `rgbdns-frag-b9c796f4839e`
- Source file: [[DNS from First Principles/Code/docs/OBSIDIAN-VAULT.md.source|docs/OBSIDIAN-VAULT.md]]
- Lines: 8-21
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-b9c796f4839e", "codeNote": "DNS from First Principles/Code/docs/OBSIDIAN-VAULT.md.source", "heading": "rgbdns-frag-b9c796f4839e: heading Rebuild", "sourcePath": "docs/OBSIDIAN-VAULT.md", "startLine": 8, "endLine": 21}
```

## Excerpt

<span id="rgbdns-frag-b9c796f4839e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b9c796f4839e: heading Rebuild

```markdown
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

```
