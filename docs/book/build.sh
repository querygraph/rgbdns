#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
central_builder=${FIRSTPAIR_BUILDER:-"$HOME/src/firstpair/publishing/scripts/build-library-book.sh"}

if [ -x "$central_builder" ]; then
  exec "$central_builder" --repo-root "$repo_root"
fi

echo "FirstPair central builder not found; using the source-owned Pandoc/Typst fallback." >&2
exec "$repo_root/docs/book/build-local.sh"

