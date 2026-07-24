---
type: "code-file"
source_path: "docs/book/build.sh"
language: "bash"
subsystem: "Documentation"
line_count: 13
fragment_count: 1
rgbdns_commit: "472c2087"
---

# docs/book/build.sh

- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]
- Source path: `docs/book/build.sh`
- Lines: 13
- Summary: Source file in the Documentation subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-ec9289abd5c6|build.sh]]: lines 1-13

## Full Source

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
