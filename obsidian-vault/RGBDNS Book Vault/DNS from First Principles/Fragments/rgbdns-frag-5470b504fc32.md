---
type: "code-fragment"
fragment_id: "rgbdns-frag-5470b504fc32"
source_path: ".github/workflows/build-book.yml"
code_note: "DNS from First Principles/Code/github/workflows/build-book.yml.source"
language: "yaml"
subsystem: "Project automation"
symbol: "build-book.yml"
kind: "file"
start_line: 1
end_line: 51
---

# build-book.yml

- Fragment ID: `rgbdns-frag-5470b504fc32`
- Source file: [[DNS from First Principles/Code/github/workflows/build-book.yml.source|.github/workflows/build-book.yml]]
- Lines: 1-51
- Subsystem: [[DNS from First Principles/Subsystems/Project automation|Project automation]]

```rgbdns-fragment
{"id": "rgbdns-frag-5470b504fc32", "codeNote": "DNS from First Principles/Code/github/workflows/build-book.yml.source", "heading": "rgbdns-frag-5470b504fc32: file build-book.yml", "sourcePath": ".github/workflows/build-book.yml", "startLine": 1, "endLine": 51}
```

## Excerpt

<span id="rgbdns-frag-5470b504fc32" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5470b504fc32: file build-book.yml

```yaml
name: Build FirstPair Book

on:
  workflow_dispatch:

permissions:
  contents: write

concurrency:
  group: build-firstpair-book
  cancel-in-progress: false

jobs:
  build:
    runs-on: macos-15
    timeout-minutes: 45
    steps:
      - name: Check out rgbdns
        uses: actions/checkout@v5

      - name: Check out FirstPair
        uses: actions/checkout@v5
        with:
          repository: firstpair/firstpair
          path: firstpair

      - name: Install the pinned publishing toolchain
        run: |
          mkdir -p "$HOME/src"
          mv firstpair "$HOME/src/firstpair"
          "$HOME/src/firstpair/publishing/scripts/install-toolchain.sh"

      - name: Build and validate the book
        run: |
          docs/book/build.sh
          docs/book/validate.sh
          "$HOME/src/firstpair/publishing/scripts/verify-library-book.sh" \
            docs/book/dist

      - name: Commit generated artifacts
        run: |
          git config user.name "First Pair Builder"
          git config user.email "builder@firstpair.org"
          git add docs/book/dist
          if git diff --cached --quiet; then
            echo "Book artifacts already current."
          else
            git commit -m "Rebuild DNS book with image cover"
            git pull --rebase origin master
            git push origin HEAD:master
          fi
```
