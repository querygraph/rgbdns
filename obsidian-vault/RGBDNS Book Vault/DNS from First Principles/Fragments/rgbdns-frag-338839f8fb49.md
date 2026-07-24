---
type: "code-fragment"
fragment_id: "rgbdns-frag-338839f8fb49"
source_path: "book.build.json"
code_note: "DNS from First Principles/Code/book.build.json.source"
language: "json"
subsystem: "Repository and build"
symbol: "book.build.json"
kind: "file"
start_line: 1
end_line: 55
---

# book.build.json

- Fragment ID: `rgbdns-frag-338839f8fb49`
- Source file: [[DNS from First Principles/Code/book.build.json.source|book.build.json]]
- Lines: 1-55
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-338839f8fb49", "codeNote": "DNS from First Principles/Code/book.build.json.source", "heading": "rgbdns-frag-338839f8fb49: file book.build.json", "sourcePath": "book.build.json", "startLine": 1, "endLine": 55}
```

## Excerpt

<span id="rgbdns-frag-338839f8fb49" class="rgbdns-fragment-target"></span>
### rgbdns-frag-338839f8fb49: file book.build.json

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
