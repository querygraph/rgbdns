---
type: "code-fragment"
fragment_id: "rgbdns-frag-44ef9d3327cb"
source_path: "README.md"
code_note: "DNS from First Principles/Code/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Book"
kind: "heading"
start_line: 188
end_line: 207
---

# Book

- Fragment ID: `rgbdns-frag-44ef9d3327cb`
- Source file: [[DNS from First Principles/Code/README.md.source|README.md]]
- Lines: 188-207
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-44ef9d3327cb", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 188, "endLine": 207}
```

## Excerpt

<span id="rgbdns-frag-44ef9d3327cb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-44ef9d3327cb: heading Book

````markdown
## Book

[*DNS from First Principles*](docs/book/rgbdns.md) develops the protocol from
names and packets through authority, recursion, DNSSEC, transfers, operations,
and security, then maps each concept to rgbdns. It also compares systemd,
runit, s6/s6-rc, OpenRC, and container-native replacements for
`svc`/`supervise`.

The committed [Obsidian reader vault](obsidian-vault/RGBDNS%20Book%20Vault)
adds a codebase-exploration part, collocates the full text/code surface, and
bundles FirstPair Reader 2 for chapter navigation and fragment cards that show
or open the referenced code lines. See [the vault guide](docs/OBSIDIAN-VAULT.md) to rebuild and validate it.

Build the FirstPair package with Pandoc and Typst:

```sh
docs/book/build.sh
docs/book/validate.sh
```

````
