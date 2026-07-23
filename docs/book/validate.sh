#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
dist="$repo_root/docs/book/dist"

for artifact in rgbdns.pdf rgbdns.epub rgbdns.html VERSION.md; do
  test -s "$dist/$artifact" || {
    echo "missing or empty artifact: $artifact" >&2
    exit 1
  }
done

pdfinfo "$dist/rgbdns.pdf" |
  grep -q '^Creator:.*Typst' || {
    echo "PDF was not rendered by Typst" >&2
    exit 1
  }

unzip -t "$dist/rgbdns.epub" >/dev/null
grep -q '<title>DNS from First Principles</title>' "$dist/rgbdns.html"

plain_words=$(pandoc "$dist/rgbdns.epub" -t plain | wc -w)
test "$plain_words" -ge 6000 || {
  echo "EPUB manuscript is unexpectedly short: $plain_words words" >&2
  exit 1
}

echo "Validated PDF, EPUB, and HTML ($plain_words words)."
