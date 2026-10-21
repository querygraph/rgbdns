---
type: "code-fragment"
fragment_id: "rgbdns-frag-adc40771268a"
source_path: "docs/OBSIDIAN-VAULT.md"
code_note: "DNS from First Principles/Code/docs/OBSIDIAN-VAULT.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Reader map"
kind: "heading"
start_line: 44
end_line: 67
---

# Reader map

- Fragment ID: `rgbdns-frag-adc40771268a`
- Source file: [[DNS from First Principles/Code/docs/OBSIDIAN-VAULT.md.source|docs/OBSIDIAN-VAULT.md]]
- Lines: 44-67
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-adc40771268a", "codeNote": "DNS from First Principles/Code/docs/OBSIDIAN-VAULT.md.source", "sourcePath": "docs/OBSIDIAN-VAULT.md", "startLine": 44, "endLine": 67}
```

## Excerpt

<span id="rgbdns-frag-adc40771268a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-adc40771268a: heading Reader map

```markdown
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
```
