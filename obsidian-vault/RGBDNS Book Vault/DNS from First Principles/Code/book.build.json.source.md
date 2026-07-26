---
type: "code-file"
source_path: "book.build.json"
language: "json"
subsystem: "Repository and build"
line_count: 55
fragment_count: 1
rgbdns_commit: "79502939"
---

# book.build.json

- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]
- Source path: `book.build.json`
- Lines: 55
- Summary: Source file in the Repository and build subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-338839f8fb49|book.build.json]]: lines 1-55

## Full Source

```json
{
  "$schema": "../firstpair/publishing/book.build.schema.json",
  "schemaVersion": 1,
  "bookRoot": "docs/book",
  "manuscript": "docs/book/rgbdns.md",
  "metadata": "docs/book/metadata.yaml",
  "cover": "docs/book/cover.md",
  "coverImage": "cover/rgbdns-cover.png",
  "headboardImage": "cover/rgbdns-headboard.png",
  "css": "docs/book/epub.css",
  "stem": "rgbdns",
  "title": "DNS from First Principles",
  "subtitle": "Names, Packets, Authority, Recursion, and the Design of rgbdns",
  "author": "Alexy Khrabrov",
  "version": {
    "source": "file",
    "file": "docs/book/VERSION"
  },
  "kindleName": "${stem} (${version})",
  "dist": "docs/book/dist",
  "buildDir": "docs/book/build/firstpair",
  "edition": "full",
  "primaryFormat": "typst",
  "mobi": false,
  "pdf": {
    "coverImage": "cover/rgbdns-cover.png",
    "coverWidth": "6in",
    "coverHeight": "9in",
    "coverFit": "cover"
  },
  "epub": {
    "coverImage": "cover/rgbdns-cover.png",
    "includeRenderedCover": false
  },
  "pandoc": {
    "reader": "markdown+smart",
    "tocDepth": 2,
    "numberSections": true,
    "resourcePaths": ["docs/book", "${repoRoot}"]
  },
  "pdfFormats": [
    {
      "name": "typst",
      "renderer": "typst",
      "stem": "rgbdns",
      "primary": true
    }
  ],
  "html": {
    "splitLevel": 1
  },
  "validators": [
    "docs/book/validate.sh"
  ]
}
```
