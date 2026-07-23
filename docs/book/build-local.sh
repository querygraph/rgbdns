#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
book_root="$repo_root/docs/book"
dist="$book_root/dist"
build="$book_root/build/firstpair"
version=$(tr -d '\r\n' < "$book_root/VERSION")

command -v pandoc >/dev/null 2>&1 || {
  echo "pandoc is required" >&2
  exit 1
}
command -v typst >/dev/null 2>&1 || {
  echo "typst is required" >&2
  exit 1
}

mkdir -p "$dist" "$build"

sources="$book_root/rgbdns.md"
common="--from=markdown+smart --metadata-file=$book_root/metadata.yaml --toc --toc-depth=2 --number-sections"

# shellcheck disable=SC2086
pandoc $common --pdf-engine=typst \
  -o "$dist/rgbdns.pdf" $sources

# shellcheck disable=SC2086
pandoc $common --css="$book_root/epub.css" \
  -o "$dist/rgbdns.epub" $sources

# shellcheck disable=SC2086
pandoc $common --standalone --embed-resources \
  --css="$book_root/epub.css" \
  -o "$dist/rgbdns.html" $sources

cat > "$dist/VERSION.md" <<EOF
# DNS from First Principles

- Version: $version
- Edition: full
- Primary renderer: Typst through Pandoc
- Formats: PDF, EPUB, standalone HTML
EOF

chmod 644 "$dist/rgbdns.pdf" "$dist/rgbdns.epub" "$dist/rgbdns.html" "$dist/VERSION.md"

echo "Built FirstPair artifacts in $dist"
