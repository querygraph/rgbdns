---
type: "code-fragment"
fragment_id: "rgbdns-frag-ec9289abd5c6"
source_path: "docs/book/build.sh"
code_note: "DNS from First Principles/Code/docs/book/build.sh.source"
language: "bash"
subsystem: "Documentation"
symbol: "build.sh"
kind: "file"
start_line: 1
end_line: 13
---

# build.sh

- Fragment ID: `rgbdns-frag-ec9289abd5c6`
- Source file: [[DNS from First Principles/Code/docs/book/build.sh.source|docs/book/build.sh]]
- Lines: 1-13
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-ec9289abd5c6", "codeNote": "DNS from First Principles/Code/docs/book/build.sh.source", "heading": "rgbdns-frag-ec9289abd5c6: file build.sh", "sourcePath": "docs/book/build.sh", "startLine": 1, "endLine": 13}
```

## Excerpt

<span id="rgbdns-frag-ec9289abd5c6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ec9289abd5c6: file build.sh

```bash
#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
central_builder=${FIRSTPAIR_BUILDER:-"$HOME/src/firstpair/publishing/scripts/build-library-book.sh"}

if [ -x "$central_builder" ]; then
  exec "$central_builder" --repo-root "$repo_root"
fi

echo "FirstPair central builder not found; using the source-owned Pandoc/Typst fallback." >&2
exec "$repo_root/docs/book/build-local.sh"

```
