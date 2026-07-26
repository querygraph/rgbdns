---
type: "code-file"
source_path: "docs/book/README.md"
language: "markdown"
subsystem: "Documentation"
line_count: 19
fragment_count: 1
rgbdns_commit: "79502939"
---

# docs/book/README.md

- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]
- Source path: `docs/book/README.md`
- Lines: 19
- Summary: Book source

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-f12840911369|Book source]]: lines 1-19

## Full Source

```markdown
# Book source

Build the complete book from the repository root:

```sh
docs/book/build.sh
```

The wrapper follows the FirstPair repository contract: it delegates to the
central builder when present and otherwise uses the checked-in Pandoc/Typst
fallback. Both paths treat `rgbdns.md` and `book.build.json` as canonical
source. A build creates `rgbdns.pdf`, `rgbdns.epub`, `rgbdns.html`, and
`VERSION.md` in `docs/book/dist/`; it never publishes them.

Validate the generated package with:

```sh
docs/book/validate.sh
```
```
