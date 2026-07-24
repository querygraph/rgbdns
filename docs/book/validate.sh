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

pdf_metadata=$(pdfinfo "$dist/rgbdns.pdf")
printf '%s\n' "$pdf_metadata" |
  grep -q '^Title:[[:space:]]*DNS from First Principles$' || {
    echo "PDF title metadata is incorrect" >&2
    exit 1
  }
printf '%s\n' "$pdf_metadata" |
  grep -q '^Author:[[:space:]]*Alexy Khrabrov$' || {
    echo "PDF author metadata is incorrect" >&2
    exit 1
  }

# A direct Typst PDF carries Creator: Typst. The FirstPair image-cover path
# prepends a separately rendered cover with pdfunite, which drops that field;
# its generated manifest instead binds the output to the verified toolchain
# and records Typst as the primary format.
if ! printf '%s\n' "$pdf_metadata" | grep -q '^Creator:.*Typst'; then
  grep -q '^primary_format:[[:space:]]*typst$' "$dist/VERSION.md" &&
    grep -q '^toolchain_lock:[[:space:]].*toolchain.lock.json$' "$dist/VERSION.md" || {
      echo "PDF has no verified Typst renderer provenance" >&2
      exit 1
    }
fi

unzip -t "$dist/rgbdns.epub" >/dev/null
grep -q '<title>DNS from First Principles</title>' "$dist/rgbdns.html"

plain_words=$(pandoc "$dist/rgbdns.epub" -t plain | wc -w)
test "$plain_words" -ge 6000 || {
  echo "EPUB manuscript is unexpectedly short: $plain_words words" >&2
  exit 1
}

echo "Validated PDF, EPUB, and HTML ($plain_words words)."
