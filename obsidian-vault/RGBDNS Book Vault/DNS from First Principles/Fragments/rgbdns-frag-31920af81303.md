---
type: "code-fragment"
fragment_id: "rgbdns-frag-31920af81303"
source_path: "README.md"
code_note: "DNS from First Principles/Code/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Book"
kind: "heading"
start_line: 44
end_line: 63
---

# Book

- Fragment ID: `rgbdns-frag-31920af81303`
- Source file: [[DNS from First Principles/Code/README.md.source|README.md]]
- Lines: 44-63
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-31920af81303", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-31920af81303: heading Book", "sourcePath": "README.md", "startLine": 44, "endLine": 63}
```

## Excerpt

<span id="rgbdns-frag-31920af81303" class="rgbdns-fragment-target"></span>
### rgbdns-frag-31920af81303: heading Book

```markdown
## Book

[*DNS from First Principles*](docs/book/rgbdns.md) develops the protocol from
names and packets through authority, recursion, DNSSEC, transfers, operations,
and security, then maps each concept to rgbdns. It also compares systemd,
runit, s6/s6-rc, OpenRC, and container-native replacements for
`svc`/`supervise`.

The committed [Obsidian reader vault](obsidian-vault/RGBDNS%20Book%20Vault)
adds a codebase-exploration part, collocates the full text/code surface, and
bundles a reader plugin for chapter navigation and prose-to-code fragment
jumps. See [the vault guide](docs/OBSIDIAN-VAULT.md) to rebuild and validate it.

Build the FirstPair package with Pandoc and Typst:

```sh
docs/book/build.sh
docs/book/validate.sh
```

```
