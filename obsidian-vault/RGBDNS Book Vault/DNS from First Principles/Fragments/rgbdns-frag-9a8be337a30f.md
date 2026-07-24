---
type: "code-fragment"
fragment_id: "rgbdns-frag-9a8be337a30f"
source_path: "FIRSTPAIR.md"
code_note: "DNS from First Principles/Code/FIRSTPAIR.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Build"
kind: "heading"
start_line: 13
end_line: 27
---

# Build

- Fragment ID: `rgbdns-frag-9a8be337a30f`
- Source file: [[DNS from First Principles/Code/FIRSTPAIR.md.source|FIRSTPAIR.md]]
- Lines: 13-27
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-9a8be337a30f", "codeNote": "DNS from First Principles/Code/FIRSTPAIR.md.source", "heading": "rgbdns-frag-9a8be337a30f: heading Build", "sourcePath": "FIRSTPAIR.md", "startLine": 13, "endLine": 27}
```

## Excerpt

<span id="rgbdns-frag-9a8be337a30f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-9a8be337a30f: heading Build

```markdown
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

```
